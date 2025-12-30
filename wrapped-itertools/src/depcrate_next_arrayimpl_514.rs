// Generated macro for impl_514 (impl)
macro_rules! Depcrate_next_arrayimpl_514 {
() => {
// Module: crate::next_array
// Provides: {"impl_514"}
// Dependencies: {}
impl < T , const N : usize > AsMut < [T] > for ArrayBuilder < T , N > { fn as_mut (& mut self) -> & mut [T] { let valid = & mut self . arr [.. self . len] ; unsafe { slice_assume_init_mut (valid) } } }
};
}
