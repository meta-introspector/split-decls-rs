// Generated macro for impl_57 (impl)
macro_rules! Depcrate_astimpl_57 {
() => {
// Module: crate::ast
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , W > fmt :: Write for DemangleContext < 'a , W > where W : 'a + DemangleWrite , { fn write_str (& mut self , s : & str) -> fmt :: Result { if s . is_empty () { return Ok (()) ; } log ! ("DemangleContext::write: '{}'" , s) ; self . out . write_string (s) . map (| _ | { self . last_char_written = s . chars () . last () ; self . bytes_written += s . len () ; }) } }
};
}
