// Generated macro for macro_43 (macro)
macro_rules! Depcrate_ruremacro_43 {
() => {
// Module: crate::rure
// Provides: {"macro_43"}
// Dependencies: {}
ffi_fn ! { fn rure_compile_must (pattern : * const c_char) -> * const Regex { let len = unsafe { CStr :: from_ptr (pattern) . to_bytes () . len () } ; let pat = pattern as * const u8 ; let mut err = Error :: new (ErrorKind :: None) ; let re = rure_compile (pat , len , RURE_DEFAULT_FLAGS , ptr :: null () , & mut err) ; if err . is_err () { let _ = writeln ! (& mut io :: stderr () , "{err}") ; let _ = writeln ! (& mut io :: stderr () , "aborting from rure_compile_must") ; unsafe { abort () } } re } }
};
}
