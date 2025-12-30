// Generated macro for blocks (function)
macro_rules! Depcrate_iter_testblocks {
() => {
// Module: crate::iter::test
// Provides: {"blocks"}
// Dependencies: {}
# [test] fn blocks () { let count = AtomicUsize :: new (0) ; let v : Vec < usize > = (0 .. 1000) . into_par_iter () . map (| _ | count . fetch_add (1 , Ordering :: Relaxed)) . by_uniform_blocks (100) . collect () ; let m = v . chunks (100) . map (| c | c . iter () . max () . copied () . unwrap ()) . collect :: < Vec < usize > > () ; assert ! (m . windows (2) . all (| w | w [0] . lt (& w [1]))) ; assert_eq ! (v . len () , 1000) ; }
};
}
