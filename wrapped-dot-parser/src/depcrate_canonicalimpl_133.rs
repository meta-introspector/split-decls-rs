// Generated macro for impl_133 (impl)
macro_rules! Depcrate_canonicalimpl_133 {
() => {
// Module: crate::canonical
// Provides: {"impl_133"}
// Dependencies: {}
# [cfg (feature = "display")] impl < A > Display for AttrStmt < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { AttrStmt :: Graph (attr) => write ! (f , "graph [{}]" , attr) , AttrStmt :: Edge (attr) => write ! (f , "edge [{}]" , attr) , AttrStmt :: Node (attr) => write ! (f , "node [{}]" , attr) , } } }
};
}
