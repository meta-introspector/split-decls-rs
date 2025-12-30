// Generated macro for fft (function)
macro_rules! Depcrate_unsffft {
() => {
// Module: crate::unsf
// Provides: {"fft"}
// Dependencies: {}
unsafe fn fft (data : * mut f64 , n : usize) { bitreversal_perm (data , n) ; radix2 (data , n , 1) ; }
};
}
