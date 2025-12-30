// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use core :: ptr ; # [link (name = "objc" , kind = "dylib")] extern "C" { } # [test] fn context_is_passed_through_correctly () { struct SyncPtr (* mut c_void) ; unsafe impl Sync for SyncPtr { } static VALUE : SyncPtr = SyncPtr (& VALUE . 0 as * const * mut c_void as * mut c_void) ; extern "C-unwind" fn check_value (value : * mut c_void) { assert_eq ! (VALUE . 0 , value) ; } let mut error : * mut c_void = ptr :: null_mut () ; let res = unsafe { try_catch (check_value , VALUE . 0 , & mut error) } ; assert_eq ! (res , 0) ; assert ! (error . is_null ()) ; } }
};
}
