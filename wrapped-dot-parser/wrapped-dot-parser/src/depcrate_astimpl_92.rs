// Generated macro for impl_92 (impl)
macro_rules! Depcrate_astimpl_92 {
() => {
// Module: crate::ast
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'a , A > TryFrom < Pair < 'a , Rule > > for Subgraph < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let mut inner = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident , Rule :: stmt_list] ,)) ? ; let id = if let Rule :: ident = inner . as_rule () { let id_str = inner . as_str () . to_string () ; inner = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: stmt_list])) ? ; Some (id_str) } else { None } ; let stmts = StmtList :: try_from (inner) ? ; Ok (Subgraph { id , stmts }) } }
};
}
