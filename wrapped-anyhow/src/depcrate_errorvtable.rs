// Generated macro for vtable (function)
macro_rules! Depcrate_errorvtable {
() => {
// Module: crate::error
// Provides: {"vtable"}
// Dependencies: {}
unsafe fn vtable (p : NonNull < ErrorImpl >) -> & 'static ErrorVTable { unsafe { * (p . as_ptr () as * const & 'static ErrorVTable) } }
};
}
