// Generated macro for impl_167 (impl)
macro_rules! Depcrate_index_setimpl_167 {
() => {
// Module: crate::index_set
// Provides: {"impl_167"}
// Dependencies: {}
impl < T , S , const N : usize > IndexSet < T , BuildHasherDefault < S > , N > { # [doc = " Creates an empty `IndexSet`"] pub const fn new () -> Self { Self { map : IndexMap :: new () , } } }
};
}
