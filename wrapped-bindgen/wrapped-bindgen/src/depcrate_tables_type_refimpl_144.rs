// Generated macro for impl_144 (impl)
macro_rules! Depcrate_tables_type_refimpl_144 {
() => {
// Module: crate::tables::type_ref
// Provides: {"impl_144"}
// Dependencies: {}
impl TypeRef { pub fn type_name (& self) -> TypeName { TypeName (self . namespace () , self . name ()) } pub fn name (& self) -> & 'static str { trim_tick (self . str (1)) } pub fn namespace (& self) -> & 'static str { self . str (2) } }
};
}
