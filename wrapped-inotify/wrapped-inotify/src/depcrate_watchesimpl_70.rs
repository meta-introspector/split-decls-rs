// Generated macro for impl_70 (impl)
macro_rules! Depcrate_watchesimpl_70 {
() => {
// Module: crate::watches
// Provides: {"impl_70"}
// Dependencies: {}
impl WatchMask { # [doc = " Wrapper around [`Self::from_bits_retain`] for backwards compatibility"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is not actually unsafe. It is just a wrapper around the"] # [doc = " safe [`Self::from_bits_retain`]."] # [deprecated = "Use the safe `from_bits_retain` method instead"] pub unsafe fn from_bits_unchecked (bits : u32) -> Self { Self :: from_bits_retain (bits) } }
};
}
