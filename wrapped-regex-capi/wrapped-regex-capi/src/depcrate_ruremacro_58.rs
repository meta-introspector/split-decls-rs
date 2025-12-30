// Generated macro for macro_58 (macro)
macro_rules! Depcrate_ruremacro_58 {
() => {
// Module: crate::rure
// Provides: {"macro_58"}
// Dependencies: {}
ffi_fn ! { fn rure_captures_new (re : * const Regex) -> * mut Captures { let re = unsafe { &* re } ; let captures = Captures (re . capture_locations ()) ; Box :: into_raw (Box :: new (captures)) } }
};
}
