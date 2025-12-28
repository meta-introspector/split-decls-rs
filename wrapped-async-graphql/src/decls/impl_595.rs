macro_rules! deps {
    () => {
        Response!();
        NextExecute!();
        ResolveInfo!();
        Extensions!();
        NextValidation!();
        ValidationFut!();
        NextRequest!();
        ResolveFut!();
        ServerError!();
        NextPrepareRequest!();
        Extension!();
        RequestFut!();
        Request!();
        NextSubscribe!();
        ServerResult!();
        ValidationResult!();
        NextParseQuery!();
        SchemaEnv!();
        Data!();
        ParseFut!();
        Result!();
        NextResolve!();
        ExtensionContext!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        # [doc (hidden)] impl Extensions { pub (crate) fn new (extensions : impl IntoIterator < Item = Arc < dyn Extension > > , schema_env : SchemaEnv , session_data : Arc < Data > ,) -> Self { Extensions { extensions : extensions . into_iter () . collect () , schema_env , session_data , query_data : None , } } # [inline] pub (crate) fn attach_query_data (& mut self , data : Arc < Data >) { self . query_data = Some (data) ; } # [inline] pub (crate) fn is_empty (& self) -> bool { self . extensions . is_empty () } # [inline] fn create_context (& self) -> ExtensionContext { ExtensionContext { schema_env : & self . schema_env , session_data : & self . session_data , query_data : self . query_data . as_deref () , } } pub async fn request (& self , request_fut : RequestFut < '_ >) -> Response { let next = NextRequest { chain : & self . extensions , request_fut , } ; next . run (& self . create_context ()) . await } pub fn subscribe < 's > (& self , stream : BoxStream < 's , Response >) -> BoxStream < 's , Response > { let next = NextSubscribe { chain : & self . extensions , } ; next . run (& self . create_context () , stream) } pub async fn prepare_request (& self , request : Request) -> ServerResult < Request > { let next = NextPrepareRequest { chain : & self . extensions , } ; next . run (& self . create_context () , request) . await } pub async fn parse_query (& self , query : & str , variables : & Variables , parse_query_fut : ParseFut < '_ > ,) -> ServerResult < ExecutableDocument > { let next = NextParseQuery { chain : & self . extensions , parse_query_fut , } ; next . run (& self . create_context () , query , variables) . await } pub async fn validation (& self , validation_fut : ValidationFut < '_ > ,) -> Result < ValidationResult , Vec < ServerError > > { let next = NextValidation { chain : & self . extensions , validation_fut , } ; next . run (& self . create_context ()) . await } pub async fn execute < 'a , 'b , F , T > (& 'a self , operation_name : Option < & str > , execute_fut_factory : F ,) -> Response where F : FnOnce (Option < Data >) -> T + Send + 'a , T : Future < Output = Response > + Send + 'a , { let next = NextExecute { chain : & self . extensions , execute_fut_factory : Box :: new (| data | execute_fut_factory (data) . boxed ()) , execute_data : None , } ; next . run (& self . create_context () , operation_name) . await } pub async fn resolve (& self , info : ResolveInfo < '_ > , resolve_fut : ResolveFut < '_ > ,) -> ServerResult < Option < Value > > { let next = NextResolve { chain : & self . extensions , resolve_fut , } ; next . run (& self . create_context () , info) . await } }
    };
}

impl_595!()