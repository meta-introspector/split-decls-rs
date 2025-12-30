// Generated macro for impl_158 (impl)
macro_rules! Depcrate_read_utilimpl_158 {
() => {
// Module: crate::read::util
// Provides: {"impl_158"}
// Dependencies: {}
# [cfg (feature = "read")] impl < T > ArrayVec < Vec < T > > { pub fn into_vec (mut self) -> Vec < T > { let len = core :: mem :: replace (& mut self . len , 0) ; let storage = core :: mem :: replace (& mut self . storage , Box :: new ([])) ; let slice = Box :: leak (storage) ; debug_assert ! (len <= slice . len ()) ; unsafe { Vec :: from_raw_parts (slice . as_mut_ptr () as * mut T , len , slice . len ()) } } }
};
}
