// Generated macro for append_dummy (function)
macro_rules! Depcrate_dummyappend_dummy {
() => {
// Module: crate::dummy
// Provides: {"append_dummy"}
// Dependencies: {}
# [doc = " Same as [`set_dummy`] but, instead of resetting, appends tokens to the"] # [doc = " existing dummy (if any). Behaves as `set_dummy` if no dummy is present."] pub fn append_dummy (dummy : TokenStream) { check_correctness () ; DUMMY_IMPL . with (| old_dummy | { let mut cell = old_dummy . borrow_mut () ; if let Some (ts) = cell . as_mut () { ts . extend (dummy) ; } else { * cell = Some (dummy) ; } }) ; }
};
}
