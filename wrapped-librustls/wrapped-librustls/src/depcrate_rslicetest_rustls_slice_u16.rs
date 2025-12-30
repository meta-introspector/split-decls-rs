// Generated macro for test_rustls_slice_u16 (function)
macro_rules! Depcrate_rslicetest_rustls_slice_u16 {
() => {
// Module: crate::rslice
// Provides: {"test_rustls_slice_u16"}
// Dependencies: {}
# [test] fn test_rustls_slice_u16 () { let u16s = vec ! [101 , 314 , 2718] ; let rsu : rustls_slice_u16 = (& * u16s) . into () ; assert_eq ! (rsu . len , 3) ; unsafe { assert_eq ! (* rsu . data , 101) ; assert_eq ! (* rsu . data . offset (1) , 314) ; assert_eq ! (* rsu . data . offset (2) , 2718) ; } }
};
}
