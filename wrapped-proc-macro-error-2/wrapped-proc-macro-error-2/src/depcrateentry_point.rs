// Generated macro for entry_point (function)
macro_rules! Depcrateentry_point {
() => {
// Module: crate
// Provides: {"entry_point"}
// Dependencies: {}
# [doc = " This is the entry point for a proc-macro."] # [doc = ""] # [doc = " **NOT PUBLIC API, SUBJECT TO CHANGE WITHOUT ANY NOTICE**"] # [doc (hidden)] pub fn entry_point < F > (f : F , proc_macro_hack : bool) -> proc_macro :: TokenStream where F : FnOnce () -> proc_macro :: TokenStream + UnwindSafe , { ENTERED_ENTRY_POINT . with (| flag | flag . set (flag . get () + 1)) ; let caught = catch_unwind (f) ; let dummy = dummy :: cleanup () ; let err_storage = imp :: cleanup () ; ENTERED_ENTRY_POINT . with (| flag | flag . set (flag . get () - 1)) ; let gen_error = | | { if proc_macro_hack { quote ! { { macro_rules ! proc_macro_call { () => (unimplemented ! ()) } # (# err_storage) * # dummy unimplemented ! () } } } else { quote ! (# (# err_storage) * # dummy) } } ; match caught { Ok (ts) => { if err_storage . is_empty () { ts } else { gen_error () . into () } } Err (boxed) => match boxed . downcast :: < AbortNow > () { Ok (_) => gen_error () . into () , Err (boxed) => resume_unwind (boxed) , } , } }
};
}
