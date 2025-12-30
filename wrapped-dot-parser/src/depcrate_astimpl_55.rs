// Generated macro for impl_55 (impl)
macro_rules! Depcrate_astimpl_55 {
() => {
// Module: crate::ast
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a , A > TryFrom < Pair < 'a , Rule > > for AttrList < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut v : Vec < AList < A > > = Vec :: new () ; let mut inners = p . clone () . into_inner () ; let alist_pair = inners . next () . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: a_list])) ? ; let alist = AList :: try_from (alist_pair) ? ; let mut tail = inners . next () . map (| p | { AttrList :: try_from (p) . map (| alist | alist . elems) . unwrap_or_default () }) . unwrap_or_default () ; v . push (alist) ; v . append (& mut tail) ; Ok (AttrList { elems : v }) } }
};
}
