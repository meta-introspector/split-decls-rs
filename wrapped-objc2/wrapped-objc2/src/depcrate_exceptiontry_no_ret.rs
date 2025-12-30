// Generated macro for try_no_ret (function)
macro_rules! Depcrate_exceptiontry_no_ret {
() => {
// Module: crate::exception
// Provides: {"try_no_ret"}
// Dependencies: {}
# [cfg (feature = "exception")] fn try_no_ret < F : FnOnce () > (closure : F) -> Result < () , Option < Retained < Exception > > > { let f = { extern "C-unwind" fn try_objc_execute_closure < F > (closure : & mut Option < F >) where F : FnOnce () , { let closure = closure . take () . unwrap () ; closure () ; } let f : extern "C-unwind" fn (& mut Option < F >) = try_objc_execute_closure ; let f : extern "C-unwind" fn (* mut c_void) = unsafe { mem :: transmute (f) } ; f } ; let mut closure = Some (closure) ; let context : * mut Option < F > = & mut closure ; let context = context . cast () ; let mut exception = ptr :: null_mut () ; let success = unsafe { objc2_exception_helper :: try_catch (f , context , & mut exception) } ; if success == 0 { Ok (()) } else { Err (unsafe { Retained :: from_raw (exception . cast ()) }) } }
};
}
