macro_rules! deps {
    () => {
        ServerError!();
        NextPrepareRequest!();
        NextSubscribe!();
        Request!();
        Result!();
        Response!();
        NextValidation!();
        NextExecute!();
        NextParseQuery!();
        NextRequest!();
        ResolveInfo!();
        ServerResult!();
        ValidationResult!();
        ExtensionContext!();
        NextResolve!();
    };
}

macro_rules! Extension {
    () => {
        deps!();
        # [doc = " Represents a GraphQL extension"] # [async_trait :: async_trait] pub trait Extension : Sync + Send + 'static { # [doc = " Called at start query/mutation request."] async fn request (& self , ctx : & ExtensionContext < '_ > , next : NextRequest < '_ >) -> Response { next . run (ctx) . await } # [doc = " Called at subscribe request."] fn subscribe < 's > (& self , ctx : & ExtensionContext < '_ > , stream : BoxStream < 's , Response > , next : NextSubscribe < '_ > ,) -> BoxStream < 's , Response > { next . run (ctx , stream) } # [doc = " Called at prepare request."] async fn prepare_request (& self , ctx : & ExtensionContext < '_ > , request : Request , next : NextPrepareRequest < '_ > ,) -> ServerResult < Request > { next . run (ctx , request) . await } # [doc = " Called at parse query."] async fn parse_query (& self , ctx : & ExtensionContext < '_ > , query : & str , variables : & Variables , next : NextParseQuery < '_ > ,) -> ServerResult < ExecutableDocument > { next . run (ctx , query , variables) . await } # [doc = " Called at validation query."] async fn validation (& self , ctx : & ExtensionContext < '_ > , next : NextValidation < '_ > ,) -> Result < ValidationResult , Vec < ServerError > > { next . run (ctx) . await } # [doc = " Called at execute query."] async fn execute (& self , ctx : & ExtensionContext < '_ > , operation_name : Option < & str > , next : NextExecute < '_ > ,) -> Response { next . run (ctx , operation_name) . await } # [doc = " Called at resolve field."] async fn resolve (& self , ctx : & ExtensionContext < '_ > , info : ResolveInfo < '_ > , next : NextResolve < '_ > ,) -> ServerResult < Option < Value > > { next . run (ctx , info) . await } }
    };
}

Extension!()