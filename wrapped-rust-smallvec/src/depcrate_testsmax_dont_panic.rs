// Generated macro for max_dont_panic (function)
macro_rules! Depcrate_testsmax_dont_panic {
() => {
// Module: crate::tests
// Provides: {"max_dont_panic"}
// Dependencies: {}
# [doc = " This assortment of tests, in combination with miri, verifies we handle UB on fishy arguments"] # [doc = " given to SmallVec. Draining and extending the allocation are fairly well-tested earlier, but"] # [doc = " `smallvec.insert(usize::MAX, val)` once slipped by!"] # [doc = ""] # [doc = " All code that indexes into SmallVecs should be tested with such \"trivially wrong\" args."] # [test] fn max_dont_panic () { let mut sv : SmallVec < i32 , 2 > = smallvec ! [0] ; let _ = sv . get (usize :: MAX) ; sv . truncate (usize :: MAX) ; }
};
}
