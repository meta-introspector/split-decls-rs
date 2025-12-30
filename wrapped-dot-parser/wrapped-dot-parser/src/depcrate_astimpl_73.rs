// Generated macro for impl_73 (impl)
macro_rules! Depcrate_astimpl_73 {
() => {
// Module: crate::ast
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a , A > TryFrom < Pair < 'a , Rule > > for EdgeStmt < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let from_pair = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: node_id , Rule :: subgraph] ,)) ? ; let rule = from_pair . as_rule () ; let from = match rule { Rule :: node_id => Either :: Left (NodeID :: try_from (from_pair) ?) , Rule :: subgraph => Either :: Right (Subgraph :: try_from (from_pair) ?) , r => { return Err (ParseError :: expect_rule (vec ! [Rule :: node_id , Rule :: subgraph] , r ,)) ; } } ; let next = inners . next () . map (EdgeRHS :: try_from) . transpose () ? . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: edge_rhs])) ? ; let attr = inners . next () . map (AttrList :: try_from) . transpose () ? ; Ok (EdgeStmt { from , next , attr }) } }
};
}
