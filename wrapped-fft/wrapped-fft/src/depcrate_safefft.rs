// Generated macro for fft (function)
macro_rules! Depcrate_safefft {
() => {
// Module: crate::safe
// Provides: {"fft"}
// Dependencies: {}
fn fft (data : & mut [f64]) { bitreversal_perm (data) ; radix2 (data , 1) ; }
};
}
