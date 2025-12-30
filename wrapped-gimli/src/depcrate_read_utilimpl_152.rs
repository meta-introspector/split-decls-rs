// Generated macro for impl_152 (impl)
macro_rules! Depcrate_read_utilimpl_152 {
() => {
// Module: crate::read::util
// Provides: {"impl_152"}
// Dependencies: {}
# [cfg (feature = "read")] unsafe impl < T , const N : usize > Sealed for Box < [T ; N] > { type Storage = Box < [MaybeUninit < T > ; N] > ; fn new_storage () -> Self :: Storage { Box :: new (unsafe { MaybeUninit :: uninit () . assume_init () }) } }
};
}
