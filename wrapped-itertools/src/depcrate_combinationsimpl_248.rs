// Generated macro for impl_248 (impl)
macro_rules! Depcrate_combinationsimpl_248 {
() => {
// Module: crate::combinations
// Provides: {"impl_248"}
// Dependencies: {}
impl < T > PoolIndex < T > for Vec < usize > { type Item = Vec < T > ; fn extract_item < I : Iterator < Item = T > > (& self , pool : & LazyBuffer < I >) -> Vec < T > where T : Clone , { pool . get_at (self) } }
};
}
