// Generated macro for bitreversal_perm (function)
macro_rules! Depcrate_safebitreversal_perm {
() => {
// Module: crate::safe
// Provides: {"bitreversal_perm"}
// Dependencies: {}
fn bitreversal_perm < T > (data : & mut [T]) { let len = data . len () / 2 ; let mut j = 1 ; for i in (1 .. data . len ()) . step_by (2) { if j > i { data . swap (j - 1 , i - 1) ; data . swap (j , i) ; } let mut m = len ; while m >= 2 && j > m { j -= m ; m >>= 1 ; } j += m ; } }
};
}
