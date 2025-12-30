// Generated macro for tests (module)
macro_rules! Depcrate___macros_seltests {
() => {
// Module: crate::__macros::sel
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: ffi :: CString ; use core :: sync :: atomic :: { AtomicBool , Ordering } ; use crate :: rc :: Retained ; use crate :: runtime :: ClassBuilder ; use crate :: runtime :: NSObject ; use crate :: { msg_send , ClassType } ; use super :: * ; # [doc = " Test the unfortunate fact that we can't use .cxx_destruct on dynamic classes."] # [test] fn test_destruct_dynamic () { static HAS_RUN : AtomicBool = AtomicBool :: new (false) ; let name = CString :: new ("TestCxxDestruct") . unwrap () ; let mut builder = ClassBuilder :: new (& name , NSObject :: class ()) . unwrap () ; unsafe extern "C" fn destruct (_ : * mut NSObject , _ : Sel) { HAS_RUN . store (true , Ordering :: Relaxed) ; } unsafe { builder . add_method (cxx_destruct_sel () , destruct as unsafe extern "C" fn (_ , _)) } ; let cls = builder . register () ; let obj : Retained < NSObject > = unsafe { msg_send ! [cls , new] } ; drop (obj) ; let has_run_destruct = HAS_RUN . load (Ordering :: Relaxed) ; if cfg ! (feature = "gnustep-1-7") { assert ! (has_run_destruct) ; } else { assert ! (! has_run_destruct) ; } } }
};
}
