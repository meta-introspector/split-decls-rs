// Generated macro for impl_247 (impl)
macro_rules! Depcrate_combinationsimpl_247 {
() => {
// Module: crate::combinations
// Provides: {"impl_247"}
// Dependencies: {}
impl < T > PoolIndex < T > for Box < [usize] > { type Item = Vec < T > ; fn extract_item < I : Iterator < Item = T > > (& self , pool : & LazyBuffer < I >) -> Vec < T > where T : Clone , { pool . get_at (self) } }
};
}
