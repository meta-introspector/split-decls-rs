// Generated macro for extract_path_ident (function)
macro_rules! Depcrate_parserextract_path_ident {
() => {
// Module: crate::parser
// Provides: {"extract_path_ident"}
// Dependencies: {}
# [doc = " Extracts the last ident from the path"] fn extract_path_ident (path : & syn :: Path) -> Result < Ident , Diagnostic > { for segment in path . segments . iter () { match segment . arguments { syn :: PathArguments :: None => { } _ => bail_span ! (path , "paths with type parameters are not supported yet") , } } match path . segments . last () { Some (value) => Ok (value . ident . clone ()) , None => { bail_span ! (path , "empty idents are not supported") ; } } }
};
}
