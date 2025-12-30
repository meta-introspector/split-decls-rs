// Generated macro for extract_parenthetical_zerovec_attrs (function)
macro_rules! Depcrate_utilsextract_parenthetical_zerovec_attrs {
() => {
// Module: crate::utils
// Provides: {"extract_parenthetical_zerovec_attrs"}
// Dependencies: {}
# [doc = " Extracts all `zerovec::name(..)` attribute"] pub fn extract_parenthetical_zerovec_attrs (attrs : & mut Vec < Attribute > , name : & str ,) -> Result < Vec < Ident > > { let mut ret = vec ! [] ; let mut error = None ; attrs . retain (| a | { let second_segment = a . path () . segments . iter () . nth (1) ; if let Some (second) = second_segment { if second . ident == name { let list = match a . parse_args :: < IdentListAttribute > () { Ok (l) => l , Err (_) => { error = Some (Error :: new (a . span () , format ! ("#[zerovec::{name}(..)] takes in a comma separated list of identifiers") ,)) ; return false ; } } ; ret . extend (list . idents . iter () . cloned ()) ; return false ; } } true }) ; if let Some (error) = error { return Err (error) ; } Ok (ret) }
};
}
