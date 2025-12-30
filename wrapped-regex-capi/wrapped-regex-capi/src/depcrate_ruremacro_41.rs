// Generated macro for macro_41 (macro)
macro_rules! Depcrate_ruremacro_41 {
() => {
// Module: crate::rure
// Provides: {"macro_41"}
// Dependencies: {}
ffi_fn ! { fn rure_iter_new (re : * const Regex ,) -> * mut Iter { Box :: into_raw (Box :: new (Iter { re : re , last_end : 0 , last_match : None , })) } }
};
}
