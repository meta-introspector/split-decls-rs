// Generated macro for rand_unicode (function)
macro_rules! Depcrate_testsrand_unicode {
() => {
// Module: crate::tests
// Provides: {"rand_unicode"}
// Dependencies: {}
# [doc = " generates random unicode strings, upto 80 chars long"] pub (crate) fn rand_unicode () -> impl Strategy < Value = String > { proptest :: collection :: vec (proptest :: char :: any () , 0 .. 80) . prop_map (| v | v . into_iter () . collect ()) }
};
}
