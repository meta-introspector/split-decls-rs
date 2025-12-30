// Generated macro for test_rustls_slice_str (function)
macro_rules! Depcrate_rslicetest_rustls_slice_str {
() => {
// Module: crate::rslice
// Provides: {"test_rustls_slice_str"}
// Dependencies: {}
# [test] fn test_rustls_slice_str () { let many_strings = vec ! ["abcd" , "" , "xyz"] ; let rss = rustls_slice_str { inner : & many_strings , } ; assert_eq ! (rustls_slice_str_len (& rss) , 3) ; assert_eq ! (rustls_slice_str_get (& rss , 0) . len , 4) ; assert_eq ! (rustls_slice_str_get (& rss , 1) . len , 0) ; assert_ne ! (rustls_slice_str_get (& rss , 1) . data , null ()) ; assert_eq ! (rustls_slice_str_get (& rss , 2) . len , 3) ; assert_eq ! (rustls_slice_str_get (& rss , 3) . len , 0) ; assert_eq ! (rustls_slice_str_get (& rss , 3) . data , null ()) ; unsafe { assert_eq ! (* rustls_slice_str_get (& rss , 0) . data , 'a' as c_char) ; assert_eq ! (* rustls_slice_str_get (& rss , 0) . data . offset (3) , 'd' as c_char) ; assert_eq ! (* rustls_slice_str_get (& rss , 2) . data , 'x' as c_char) ; assert_eq ! (* rustls_slice_str_get (& rss , 2) . data . offset (2) , 'z' as c_char) ; } }
};
}
