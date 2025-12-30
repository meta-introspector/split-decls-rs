// Generated macro for low_bits (function)
macro_rules! Depcrate_atomiclow_bits {
() => {
// Module: crate::atomic
// Provides: {"low_bits"}
// Dependencies: {}
# [doc = " Returns a bitmask containing the unused least significant bits of an aligned pointer to `T`."] # [inline] fn low_bits < T : ? Sized + Pointable > () -> usize { (1 << T :: ALIGN . trailing_zeros ()) - 1 }
};
}
