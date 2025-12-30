// Generated macro for get_from_cargo (function)
macro_rules! Depcrate_internals_cratenameget_from_cargo {
() => {
// Module: crate::internals::cratename
// Provides: {"get_from_cargo"}
// Dependencies: {}
pub (crate) fn get_from_cargo () -> Ident { let name = & crate_name (BORSH) . unwrap_or_else (| err | panic ! ("`proc_macro_crate::crate_name` call error: {:#?}" , err)) ; let name = match name { FoundCrate :: Itself => BORSH , FoundCrate :: Name (name) => name . as_str () , } ; Ident :: new (name , Span :: call_site ()) }
};
}
