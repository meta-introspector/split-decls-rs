// Generated macro for impl_59 (impl)
macro_rules! Depcrate_ensureimpl_59 {
() => {
// Module: crate::ensure
// Provides: {"impl_59"}
// Dependencies: {}
impl Buf { fn new () -> Self { Buf { bytes : [MaybeUninit :: uninit () ; 40] , written : 0 , } } fn as_str (& self) -> & str { unsafe { str :: from_utf8_unchecked (slice :: from_raw_parts (self . bytes . as_ptr () . cast :: < u8 > () , self . written ,)) } } }
};
}
