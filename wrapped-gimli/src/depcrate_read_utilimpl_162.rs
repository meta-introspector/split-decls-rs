// Generated macro for impl_162 (impl)
macro_rules! Depcrate_read_utilimpl_162 {
() => {
// Module: crate::read::util
// Provides: {"impl_162"}
// Dependencies: {}
impl < A : ArrayLike > ops :: DerefMut for ArrayVec < A > { fn deref_mut (& mut self) -> & mut [A :: Item] { let slice = & mut A :: as_mut_slice (& mut self . storage) ; debug_assert ! (self . len <= slice . len ()) ; unsafe { slice :: from_raw_parts_mut (slice . as_mut_ptr () as _ , self . len) } } }
};
}
