// Generated macro for ifft (function)
macro_rules! Depcrate_unsfifft {
() => {
// Module: crate::unsf
// Provides: {"ifft"}
// Dependencies: {}
unsafe fn ifft (data : * mut f64 , n : usize) { bitreversal_perm (data , n) ; radix2 (data , n , - 1) ; rescale (data , n) ; }
};
}
