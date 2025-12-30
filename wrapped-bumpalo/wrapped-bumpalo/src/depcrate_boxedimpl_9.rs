// Generated macro for impl_9 (impl)
macro_rules! Depcrate_boxedimpl_9 {
() => {
// Module: crate::boxed
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > Default for Box < 'a , str > { fn default () -> Box < 'a , str > { unsafe { Box :: from_raw (Box :: into_raw (Box :: < [u8] > :: default ()) as * mut str) } } }
};
}
