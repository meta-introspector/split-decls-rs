// Generated macro for ifft (function)
macro_rules! Depcrate_safeifft {
() => {
// Module: crate::safe
// Provides: {"ifft"}
// Dependencies: {}
fn ifft (data : & mut [f64]) { bitreversal_perm (data) ; radix2 (data , - 1) ; rescale (data , data . len () / 2) ; }
};
}
