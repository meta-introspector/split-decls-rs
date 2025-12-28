macro_rules! deps {
    () => {
        QueryEnvInner!();
        ContextBase!();
        QueryPathNode!();
        QueryEnv!();
        Data!();
        SchemaEnv!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl QueryEnv { # [doc (hidden)] pub fn new (inner : QueryEnvInner) -> QueryEnv { QueryEnv (Arc :: new (inner)) } # [doc (hidden)] pub fn create_context < 'a , T > (& 'a self , schema_env : & 'a SchemaEnv , path_node : Option < QueryPathNode < 'a > > , item : T , execute_data : Option < & 'a Data > ,) -> ContextBase < 'a , T > { ContextBase { path_node , is_for_introspection : false , item , schema_env , query_env : self , execute_data , } } }
    };
}

impl_349!()