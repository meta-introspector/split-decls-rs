macro_rules! deps {
    () => {
        NextExecute!();
        Query!();
        LoggerExtension!();
        Response!();
        Error!();
        PathSegment!();
        Field!();
        Extension!();
        ExtensionContext!();
        NextParseQuery!();
        ServerResult!();
    };
}

macro_rules! impl_549 {
    () => {
        deps!();
        # [async_trait :: async_trait] impl Extension for LoggerExtension { async fn parse_query (& self , ctx : & ExtensionContext < '_ > , query : & str , variables : & Variables , next : NextParseQuery < '_ > ,) -> ServerResult < ExecutableDocument > { let document = next . run (ctx , query , variables) . await ? ; let is_schema = document . operations . iter () . filter (| (_ , operation) | operation . node . ty == OperationType :: Query) . any (| (_ , operation) | operation . node . selection_set . node . items . iter () . any (| selection | matches ! (& selection . node , Selection :: Field (field) if field . node . name . node == "__schema"))) ; if ! is_schema { log :: info ! (target : "async-graphql" , "[Execute] {}" , ctx . stringify_execute_doc (& document , variables)) ; } Ok (document) } async fn execute (& self , ctx : & ExtensionContext < '_ > , operation_name : Option < & str > , next : NextExecute < '_ > ,) -> Response { let resp = next . run (ctx , operation_name) . await ; if resp . is_err () { for err in & resp . errors { if ! err . path . is_empty () { let mut path = String :: new () ; for (idx , s) in err . path . iter () . enumerate () { if idx > 0 { path . push ('.') ; } match s { PathSegment :: Index (idx) => { let _ = write ! (& mut path , "{}" , idx) ; } PathSegment :: Field (name) => { let _ = write ! (& mut path , "{}" , name) ; } } } log :: info ! (target : "async-graphql" , "[Error] path={} message={}" , path , err . message ,) ; } else { log :: info ! (target : "async-graphql" , "[Error] message={}" , err . message ,) ; } } } resp } }
    };
}

impl_549!()