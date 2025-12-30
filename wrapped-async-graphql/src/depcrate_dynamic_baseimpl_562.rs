// Generated macro for impl_562 (impl)
macro_rules! Depcrate_dynamic_baseimpl_562 {
() => {
// Module: crate::dynamic::base
// Provides: {"impl_562"}
// Dependencies: {}
impl BaseContainer for Interface { type FieldType = InterfaceField ; # [inline] fn name (& self) -> & str { & self . name } fn graphql_type (& self) -> & str { "Interface" } # [inline] fn field (& self , name : & str) -> Option < & Self :: FieldType > { self . fields . get (name) } }
};
}
