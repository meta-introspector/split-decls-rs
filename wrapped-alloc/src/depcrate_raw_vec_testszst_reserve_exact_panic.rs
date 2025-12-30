// Generated macro for zst_reserve_exact_panic (function)
macro_rules! Depcrate_raw_vec_testszst_reserve_exact_panic {
() => {
// Module: crate::raw_vec::tests
// Provides: {"zst_reserve_exact_panic"}
// Dependencies: {}
# [test] # [should_panic (expected = "capacity overflow")] fn zst_reserve_exact_panic () { let mut v : RawVec < ZST > = RawVec :: new () ; zst_sanity (& v) ; v . reserve_exact (101 , usize :: MAX - 100) ; }
};
}
