// Generated macro for ident (function)
macro_rules! Depcrate_parseident {
() => {
// Module: crate::parse
// Provides: {"ident"}
// Dependencies: {}
fn ident (input : Cursor) -> PResult < crate :: Ident > { if ["r\"" , "r#\"" , "r##" , "b\"" , "b\'" , "br\"" , "br#" , "c\"" , "cr\"" , "cr#" ,] . iter () . any (| prefix | input . starts_with (prefix)) { Err (Reject) } else { ident_any (input) } }
};
}
