// Generated macro for load_token_file (function)
macro_rules! Depcrate_parseload_token_file {
() => {
// Module: crate::parse
// Provides: {"load_token_file"}
// Dependencies: {}
fn load_token_file (relative_to_workspace_root : impl AsRef < Path > ,) -> Result < BTreeMap < String , String > > { let path = workspace_path :: get (relative_to_workspace_root) ; let src = fs :: read_to_string (path) ? ; let file = syn :: parse_file (& src) ? ; for item in file . items { if let Item :: Macro (item) = item { match item . ident { Some (i) if i == "Token" => { } _ => continue , } let tokens = item . mac . parse_body_with (parsing :: parse_token_macro) ? ; return Ok (tokens) ; } } panic ! ("failed to parse Token macro") }
};
}
