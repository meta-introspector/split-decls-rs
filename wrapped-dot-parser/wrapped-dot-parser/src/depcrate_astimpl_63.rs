// Generated macro for impl_63 (impl)
macro_rules! Depcrate_astimpl_63 {
() => {
// Module: crate::ast
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > TryFrom < Pair < 'a , Rule > > for AList < (ID < 'a > , ID < 'a >) > { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut v = Vec :: new () ; let mut inners = p . clone () . into_inner () ; let p_id1 = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident])) ? ; let id1 = ID :: try_from (p_id1) ? ; let p_id2 = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident])) ? ; let id2 = ID :: try_from (p_id2) ? ; let mut tail = inners . next () . map (| p | { AList :: try_from (p) . map (| alist | alist . elems) . unwrap_or_default () }) . unwrap_or_default () ; v . push ((id1 , id2)) ; v . append (& mut tail) ; Ok (AList { elems : v }) } }
};
}
