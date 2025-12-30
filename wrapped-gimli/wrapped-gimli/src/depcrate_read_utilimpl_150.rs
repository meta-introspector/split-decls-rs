// Generated macro for impl_150 (impl)
macro_rules! Depcrate_read_utilimpl_150 {
() => {
// Module: crate::read::util
// Provides: {"impl_150"}
// Dependencies: {}
unsafe impl < T , const N : usize > Sealed for [T ; N] { type Storage = [MaybeUninit < T > ; N] ; fn new_storage () -> Self :: Storage { unsafe { MaybeUninit :: uninit () . assume_init () } } }
};
}
