// Generated macro for impl_40 (impl)
macro_rules! Depcrate_astimpl_40 {
() => {
// Module: crate::ast
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a , A > TryFrom < Pair < 'a , Rule > > for StmtList < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let inner = p . into_inner () ; let mut stmts = Vec :: new () ; for stmt in inner { stmts . push (Stmt :: try_from (stmt) ?) ; } Ok (StmtList { stmts }) } }
};
}
