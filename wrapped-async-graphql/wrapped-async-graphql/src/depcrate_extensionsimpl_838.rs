// Generated macro for impl_838 (impl)
macro_rules! Depcrate_extensionsimpl_838 {
() => {
// Module: crate::extensions
// Provides: {"impl_838"}
// Dependencies: {}
impl NextParseQuery < '_ > { # [doc = " Call the [Extension::parse_query] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > , query : & str , variables : & Variables ,) -> ServerResult < ExecutableDocument > { if let Some ((first , next)) = self . chain . split_first () { first . parse_query (ctx , query , variables , NextParseQuery { chain : next , parse_query_fut : self . parse_query_fut , } ,) . await } else { self . parse_query_fut . await } } }
};
}
