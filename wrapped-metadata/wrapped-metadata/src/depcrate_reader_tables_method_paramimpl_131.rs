// Generated macro for impl_131 (impl)
macro_rules! Depcrate_reader_tables_method_paramimpl_131 {
() => {
// Module: crate::reader::tables::method_param
// Provides: {"impl_131"}
// Dependencies: {}
impl MethodParam < '_ > { pub fn flags (& self) -> ParamAttributes { ParamAttributes (self . usize (0) . try_into () . unwrap ()) } pub fn sequence (& self) -> u16 { self . usize (1) . try_into () . unwrap () } pub fn name (& self) -> & str { self . str (2) } }
};
}
