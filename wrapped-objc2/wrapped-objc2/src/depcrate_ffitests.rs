// Generated macro for tests (module)
macro_rules! Depcrate_ffitests {
() => {
// Module: crate::ffi
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use core :: ffi :: CStr ; # [test] fn smoke () { let name = CStr :: from_bytes_with_nul (b"abc:def:\0") . unwrap () ; let sel = unsafe { sel_registerName (name . as_ptr ()) . unwrap () } ; let rtn = unsafe { CStr :: from_ptr (sel_getName (sel)) } ; assert_eq ! (name , rtn) ; } }
};
}
