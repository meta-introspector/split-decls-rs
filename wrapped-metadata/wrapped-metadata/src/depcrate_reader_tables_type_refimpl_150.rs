// Generated macro for impl_150 (impl)
macro_rules! Depcrate_reader_tables_type_refimpl_150 {
() => {
// Module: crate::reader::tables::type_ref
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'a > TypeRef < 'a > { pub fn scope (& self) -> ResolutionScope < 'a > { self . decode (0) } pub fn name (& self) -> & 'a str { self . str (1) } pub fn namespace (& self) -> & 'a str { self . str (2) } }
};
}
