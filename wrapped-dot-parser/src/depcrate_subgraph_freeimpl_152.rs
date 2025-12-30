// Generated macro for impl_152 (impl)
macro_rules! Depcrate_subgraph_freeimpl_152 {
() => {
// Module: crate::subgraph_free
// Provides: {"impl_152"}
// Dependencies: {}
impl < A > From < crate :: ast :: StmtList < A > > for StmtList < A > where A : Clone , { fn from (stmts : crate :: ast :: StmtList < A >) -> Self { let mut v = Vec :: new () ; for stmt in stmts { match stmt { crate :: ast :: Stmt :: NodeStmt (n) => { v . push (Stmt :: NodeStmt (n)) ; } crate :: ast :: Stmt :: EdgeStmt (e) => { let edges : Vec < EdgeStmt < A > > = e . into () ; for stmt in edges { v . push (Stmt :: EdgeStmt (stmt)) ; } } crate :: ast :: Stmt :: AttrStmt (a) => { v . push (Stmt :: AttrStmt (a)) ; } crate :: ast :: Stmt :: IDEq (s1 , s2) => { v . push (Stmt :: IDEq (s1 , s2)) ; } crate :: ast :: Stmt :: Subgraph (graph) => { let stmts : StmtList < A > = graph . stmts . into () ; for stmt in stmts { v . push (stmt) } } } } Self { stmts : v } } }
};
}
