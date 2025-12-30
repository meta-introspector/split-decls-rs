// Generated macro for tests (module)
macro_rules! Depcrate_rawtests {
() => {
// Module: crate::raw
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_ptr2str () { unsafe { { let cstr = b"\0" ; let rstr = ptr2str (cstr as * const _ as * const c_char) ; assert_eq ! (rstr . len () , 1) ; assert_eq ! (rstr , b"\0") ; } { let cstr = b"foo  baaar\0" ; let rstr = ptr2str (cstr as * const _ as * const c_char) ; assert_eq ! (rstr . len () , b"foo  baaar\0" . len ()) ; assert_eq ! (rstr , b"foo  baaar\0") ; } } } }
};
}
