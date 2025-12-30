// Generated macro for impl_81 (impl)
macro_rules! Depcrate_astimpl_81 {
() => {
// Module: crate::ast
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a , A > TryFrom < Pair < 'a , Rule > > for NodeStmt < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let node = inners . next () . map (NodeID :: try_from) . transpose () ? . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: node_id])) ? ; let attr = inners . next () . map (AttrList :: try_from) . transpose () ? ; Ok (NodeStmt { node , attr }) } }
};
}
