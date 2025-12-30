// Generated macro for impl_72 (impl)
macro_rules! Depcrate_iterimpl_72 {
() => {
// Module: crate::iter
// Provides: {"impl_72"}
// Dependencies: {}
impl < T , N : ArrayLength > Drop for GenericArrayIter < T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . as_mut_slice ()) ; } } }
};
}
