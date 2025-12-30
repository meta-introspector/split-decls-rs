// Generated macro for bitreversal_perm (function)
macro_rules! Depcrate_unsfbitreversal_perm {
() => {
// Module: crate::unsf
// Provides: {"bitreversal_perm"}
// Dependencies: {}
unsafe fn bitreversal_perm (data : * mut f64 , len : usize) { let mut j = 1 ; for i in (1 .. 2 * len) . step_by (2) { if j > i { std :: ptr :: swap (data . add (j - 1) , data . add (i - 1)) ; std :: ptr :: swap (data . add (j) , data . add (i)) ; } let mut m = len ; while m >= 2 && j > m { j -= m ; m >>= 1 ; } j += m ; } }
};
}
