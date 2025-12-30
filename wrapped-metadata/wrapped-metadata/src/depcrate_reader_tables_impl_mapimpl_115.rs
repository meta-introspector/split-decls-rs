// Generated macro for impl_115 (impl)
macro_rules! Depcrate_reader_tables_impl_mapimpl_115 {
() => {
// Module: crate::reader::tables::impl_map
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a > ImplMap < 'a > { pub fn flags (& self) -> PInvokeAttributes { PInvokeAttributes (self . usize (0) . try_into () . unwrap ()) } pub fn import_name (& self) -> & str { self . str (2) } pub fn import_scope (& self) -> ModuleRef < 'a > { self . row (3) } }
};
}
