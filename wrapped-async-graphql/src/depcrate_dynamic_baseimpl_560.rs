// Generated macro for impl_560 (impl)
macro_rules! Depcrate_dynamic_baseimpl_560 {
() => {
// Module: crate::dynamic::base
// Provides: {"impl_560"}
// Dependencies: {}
impl BaseContainer for Object { type FieldType = Field ; # [inline] fn name (& self) -> & str { & self . name } fn graphql_type (& self) -> & str { "Object" } # [inline] fn field (& self , name : & str) -> Option < & Self :: FieldType > { self . fields . get (name) } }
};
}
