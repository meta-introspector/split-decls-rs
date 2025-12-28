macro_rules! deps {
    () => {
        ServerResult!();
        ExtensionContext!();
        NextParseQuery!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        impl NextParseQuery < '_ > { # [doc = " Call the [Extension::parse_query] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > , query : & str , variables : & Variables ,) -> ServerResult < ExecutableDocument > { if let Some ((first , next)) = self . chain . split_first () { first . parse_query (ctx , query , variables , NextParseQuery { chain : next , parse_query_fut : self . parse_query_fut , } ,) . await } else { self . parse_query_fut . await } } }
    };
}

impl_585!()