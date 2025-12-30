// Generated macro for impl_741 (impl)
macro_rules! Depcrate_csrimpl_741 {
() => {
// Module: crate::csr
// Provides: {"impl_741"}
// Dependencies: {}
impl < N : Clone , E : Clone , Ty , Ix : Clone > Clone for Csr < N , E , Ty , Ix > { fn clone (& self) -> Self { Csr { column : self . column . clone () , edges : self . edges . clone () , row : self . row . clone () , node_weights : self . node_weights . clone () , edge_count : self . edge_count , ty : self . ty , } } }
};
}
