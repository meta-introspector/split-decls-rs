// Generated macro for impl_111 (impl)
macro_rules! Depcrate_canonicalimpl_111 {
() => {
// Module: crate::canonical
// Provides: {"impl_111"}
// Dependencies: {}
impl < A , G > From < G > for Graph < A > where G : Into < SFGraph < A > > , A : Clone , { fn from (graph : G) -> Self { let graph = graph . into () ; let mut attrs : Vec < AstAttrStmt < _ > > = Vec :: new () ; let mut nodes = Vec :: new () ; let mut edges = Vec :: new () ; let mut ideqs = Vec :: new () ; for stmt in graph . stmts { match stmt { Stmt :: NodeStmt (node) => nodes . push (node) , Stmt :: EdgeStmt (edge) => edges . push (edge) , Stmt :: AttrStmt (attr) => attrs . push (attr) , Stmt :: IDEq (lhs , rhs) => ideqs . push (IDEq { lhs , rhs }) , } } let mut nodes : NodeSet < A > = nodes . into () ; let edges : EdgeSet < A > = (edges , & mut nodes) . into () ; let attr : Vec < AttrStmt < A > > = attrs . into_iter () . flat_map (| stmt : AstAttrStmt < _ > | { let attr_l : Vec < AttrStmt < A > > = stmt . into () ; attr_l }) . collect () ; Graph { strict : graph . strict , is_digraph : graph . is_digraph , name : graph . name , attr , nodes , edges , ideqs , } } }
};
}
