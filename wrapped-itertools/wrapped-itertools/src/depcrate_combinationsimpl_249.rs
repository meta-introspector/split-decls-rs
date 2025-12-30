// Generated macro for impl_249 (impl)
macro_rules! Depcrate_combinationsimpl_249 {
() => {
// Module: crate::combinations
// Provides: {"impl_249"}
// Dependencies: {}
impl < T , const K : usize > PoolIndex < T > for [usize ; K] { type Item = [T ; K] ; fn extract_item < I : Iterator < Item = T > > (& self , pool : & LazyBuffer < I >) -> [T ; K] where T : Clone , { pool . get_array (* self) } }
};
}
