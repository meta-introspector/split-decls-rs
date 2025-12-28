macro_rules! deps {
    () => {
        ExtensionContext!();
        ResolveInfo!();
        ServerResult!();
        ApolloTracingExtension!();
        Extension!();
        ResolveState!();
        NextResolve!();
        NextExecute!();
        Response!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        # [async_trait :: async_trait] impl Extension for ApolloTracingExtension { async fn execute (& self , ctx : & ExtensionContext < '_ > , operation_name : Option < & str > , next : NextExecute < '_ > ,) -> Response { self . inner . lock () . await . start_time = Utc :: now () ; let resp = next . run (ctx , operation_name) . await ; let mut inner = self . inner . lock () . await ; inner . end_time = Utc :: now () ; inner . resolves . sort_by (| a , b | a . start_offset . cmp (& b . start_offset)) ; resp . extension ("tracing" , value ! ({ "version" : 1 , "startTime" : inner . start_time . to_rfc3339 () , "endTime" : inner . end_time . to_rfc3339 () , "duration" : (inner . end_time - inner . start_time) . num_nanoseconds () , "execution" : { "resolvers" : inner . resolves } }) ,) } async fn resolve (& self , ctx : & ExtensionContext < '_ > , info : ResolveInfo < '_ > , next : NextResolve < '_ > ,) -> ServerResult < Option < Value > > { let path = info . path_node . to_string_vec () ; let field_name = info . path_node . field_name () . to_string () ; let parent_type = info . parent_type . to_string () ; let return_type = info . return_type . to_string () ; let start_time = Utc :: now () ; let start_offset = (start_time - self . inner . lock () . await . start_time) . num_nanoseconds () . unwrap () ; let res = next . run (ctx , info) . await ; let end_time = Utc :: now () ; self . inner . lock () . await . resolves . push (ResolveState { path , field_name , parent_type , return_type , start_time , end_time , start_offset , }) ; res } }
    };
}

impl_544!();