// Generated macro for rand_unicode_collection (function)
macro_rules! Depcrate_testsrand_unicode_collection {
() => {
// Module: crate::tests
// Provides: {"rand_unicode_collection"}
// Dependencies: {}
# [doc = " generates groups upto 40 strings long of random unicode strings, upto 80 chars long"] fn rand_unicode_collection () -> impl Strategy < Value = Vec < String > > { proptest :: collection :: vec (rand_unicode () , 0 .. 40) }
};
}
