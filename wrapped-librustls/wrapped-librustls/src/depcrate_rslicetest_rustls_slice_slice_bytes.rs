// Generated macro for test_rustls_slice_slice_bytes (function)
macro_rules! Depcrate_rslicetest_rustls_slice_slice_bytes {
() => {
// Module: crate::rslice
// Provides: {"test_rustls_slice_slice_bytes"}
// Dependencies: {}
# [test] fn test_rustls_slice_slice_bytes () { let many_bytes : Vec < & [u8] > = vec ! [b"abcd" , b"" , b"xyz"] ; let rssb = rustls_slice_slice_bytes { inner : & many_bytes } ; assert_eq ! (rustls_slice_slice_bytes_len (& rssb) , 3) ; assert_eq ! (rustls_slice_slice_bytes_get (& rssb , 0) . len , 4) ; assert_eq ! (rustls_slice_slice_bytes_get (& rssb , 1) . len , 0) ; assert_ne ! (rustls_slice_slice_bytes_get (& rssb , 1) . data , null ()) ; assert_eq ! (rustls_slice_slice_bytes_get (& rssb , 2) . len , 3) ; assert_eq ! (rustls_slice_slice_bytes_get (& rssb , 3) . len , 0) ; assert_eq ! (rustls_slice_slice_bytes_get (& rssb , 3) . data , null ()) ; unsafe { assert_eq ! (* rustls_slice_slice_bytes_get (& rssb , 0) . data , b'a') ; assert_eq ! (* rustls_slice_slice_bytes_get (& rssb , 0) . data . offset (3) , b'd') ; assert_eq ! (* rustls_slice_slice_bytes_get (& rssb , 2) . data , b'x') ; assert_eq ! (* rustls_slice_slice_bytes_get (& rssb , 2) . data . offset (2) , b'z') ; } }
};
}
