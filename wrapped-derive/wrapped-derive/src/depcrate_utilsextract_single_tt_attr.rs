// Generated macro for extract_single_tt_attr (function)
macro_rules! Depcrate_utilsextract_single_tt_attr {
() => {
// Module: crate::utils
// Provides: {"extract_single_tt_attr"}
// Dependencies: {}
pub fn extract_single_tt_attr (attrs : & mut Vec < Attribute > , name : & str ,) -> Result < Option < TokenStream2 > > { let mut ret = None ; let mut error = None ; attrs . retain (| a | { let second_segment = a . path () . segments . iter () . nth (1) ; if let Some (second) = second_segment { if second . ident == name { if ret . is_some () { error = Some (Error :: new (a . span () , "Can only specify a single VarZeroVecFormat via #[zerovec::format(..)]" ,)) ; return false } ret = match a . parse_args :: < TokenStream2 > () { Ok (l) => Some (l) , Err (_) => { error = Some (Error :: new (a . span () , format ! ("#[zerovec::{name}(..)] takes in a comma separated list of identifiers") ,)) ; return false ; } } ; return false ; } } true }) ; if let Some (error) = error { return Err (error) ; } Ok (ret) }
};
}
