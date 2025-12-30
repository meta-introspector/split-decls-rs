// Generated macro for get_doc_string_in_attr (function)
macro_rules! Depcrate_documentationget_doc_string_in_attr {
() => {
// Module: crate::documentation
// Provides: {"get_doc_string_in_attr"}
// Dependencies: {}
fn get_doc_string_in_attr (it : & ast :: Attr) -> Option < ast :: String > { match it . expr () { Some (ast :: Expr :: Literal (lit)) => match lit . kind () { ast :: LiteralKind :: String (it) => Some (it) , _ => None , } , None => { None } _ => None , } }
};
}
