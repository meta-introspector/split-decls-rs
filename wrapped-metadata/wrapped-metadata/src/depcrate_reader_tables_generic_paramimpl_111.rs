// Generated macro for impl_111 (impl)
macro_rules! Depcrate_reader_tables_generic_paramimpl_111 {
() => {
// Module: crate::reader::tables::generic_param
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'a > GenericParam < 'a > { pub fn sequence (& self) -> u16 { self . usize (0) . try_into () . unwrap () } pub fn flags (& self) -> GenericParamAttributes { GenericParamAttributes (self . usize (1) . try_into () . unwrap ()) } pub fn owner (& self) -> TypeOrMethodDef < 'a > { self . decode (2) } pub fn name (& self) -> & str { self . str (3) } }
};
}
