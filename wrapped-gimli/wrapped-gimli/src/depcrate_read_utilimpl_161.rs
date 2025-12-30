// Generated macro for impl_161 (impl)
macro_rules! Depcrate_read_utilimpl_161 {
() => {
// Module: crate::read::util
// Provides: {"impl_161"}
// Dependencies: {}
impl < A : ArrayLike > ops :: Deref for ArrayVec < A > { type Target = [A :: Item] ; fn deref (& self) -> & [A :: Item] { let slice = & A :: as_slice (& self . storage) ; debug_assert ! (self . len <= slice . len ()) ; unsafe { slice :: from_raw_parts (slice . as_ptr () as _ , self . len) } } }
};
}
