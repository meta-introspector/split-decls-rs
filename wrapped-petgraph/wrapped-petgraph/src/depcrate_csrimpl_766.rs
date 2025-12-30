// Generated macro for impl_766 (impl)
macro_rules! Depcrate_csrimpl_766 {
() => {
// Module: crate::csr
// Provides: {"impl_766"}
// Dependencies: {}
impl < N , E , Ty , Ix > Index < NodeIndex < Ix > > for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = N ; fn index (& self , ix : NodeIndex < Ix >) -> & N { & self . node_weights [ix . index ()] } }
};
}
