// Generated macro for flatten (function)
macro_rules! Depcrate_impls_nalgebra_v0_33flatten {
() => {
// Module: crate::impls::nalgebra_v0_33
// Provides: {"flatten"}
// Dependencies: {}
# [doc = " Const version of the const-unstable [`<[[T; N]]>::as_flattened()`]"] const fn flatten < T , const N : usize > (slice : & [[T ; N]]) -> & [T] { const { assert ! (size_of ::< T > () != 0) ; } let len = unsafe { slice . len () . unchecked_mul (N) } ; unsafe { core :: slice :: from_raw_parts (slice . as_ptr () . cast () , len) } }
};
}
