// Generated macro for macro_71 (macro)
macro_rules! Depcrate_ruremacro_71 {
() => {
// Module: crate::rure
// Provides: {"macro_71"}
// Dependencies: {}
ffi_fn ! { fn rure_escape_must (pattern : * const c_char) -> * const c_char { let len = unsafe { CStr :: from_ptr (pattern) . to_bytes () . len () } ; let pat = pattern as * const u8 ; let mut err = Error :: new (ErrorKind :: None) ; let esc = rure_escape (pat , len , & mut err) ; if err . is_err () { let _ = writeln ! (& mut io :: stderr () , "{err}") ; let _ = writeln ! (& mut io :: stderr () , "aborting from rure_escape_must") ; unsafe { abort () } } esc } }
};
}
