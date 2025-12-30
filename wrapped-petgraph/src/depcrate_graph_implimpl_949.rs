// Generated macro for impl_949 (impl)
macro_rules! Depcrate_graph_implimpl_949 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_949"}
// Dependencies: {}
# [doc = " The resulting cloned graph has the same graph indices as `self`."] impl < N , E , Ty , Ix > Clone for Graph < N , E , Ty , Ix > where N : Clone , E : Clone , Ix : Copy , { fn clone (& self) -> Self { Graph { nodes : self . nodes . clone () , edges : self . edges . clone () , ty : self . ty , } } fn clone_from (& mut self , rhs : & Self) { self . nodes . clone_from (& rhs . nodes) ; self . edges . clone_from (& rhs . edges) ; self . ty = rhs . ty ; } }
};
}
