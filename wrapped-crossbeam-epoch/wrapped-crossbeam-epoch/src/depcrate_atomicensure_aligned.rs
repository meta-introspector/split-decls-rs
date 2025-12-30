// Generated macro for ensure_aligned (function)
macro_rules! Depcrate_atomicensure_aligned {
() => {
// Module: crate::atomic
// Provides: {"ensure_aligned"}
// Dependencies: {}
# [doc = " Panics if the pointer is not properly unaligned."] # [inline] fn ensure_aligned < T : ? Sized + Pointable > (raw : * mut ()) { assert_eq ! (raw as usize & low_bits ::< T > () , 0 , "unaligned pointer") ; }
};
}
