// Generated macro for impl_767 (impl)
macro_rules! Depcrate_csrimpl_767 {
() => {
// Module: crate::csr
// Provides: {"impl_767"}
// Dependencies: {}
impl < N , E , Ty , Ix > IndexMut < NodeIndex < Ix > > for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , ix : NodeIndex < Ix >) -> & mut N { & mut self . node_weights [ix . index ()] } }
};
}
