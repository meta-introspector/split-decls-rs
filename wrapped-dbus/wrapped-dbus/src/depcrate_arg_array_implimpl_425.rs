// Generated macro for impl_425 (impl)
macro_rules! Depcrate_arg_array_implimpl_425 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_425"}
// Dependencies: {}
impl < 'a , T : Get < 'a > > Iterator for Array < 'a , T , Iter < 'a > > { type Item = T ; fn next (& mut self) -> Option < T > { let i = self . 0 . get () ; self . 0 . next () ; i } }
};
}
