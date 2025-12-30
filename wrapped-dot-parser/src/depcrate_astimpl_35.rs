// Generated macro for impl_35 (impl)
macro_rules! Depcrate_astimpl_35 {
() => {
// Module: crate::ast
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , A > TryFrom < Pair < 'a , Rule > > for Graphs < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inner = p . clone () . into_inner () . into_iter () ; let first_graph_pair = inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: dotfile])) ? ; let mut graphs = vec ! [Graph :: try_from (first_graph_pair) ?] ; for p in inner { graphs . push (Graph :: try_from (p) ?) ; } Ok (Graphs { graphs }) } }
};
}
