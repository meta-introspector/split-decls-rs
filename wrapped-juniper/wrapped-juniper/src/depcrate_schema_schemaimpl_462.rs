// Generated macro for impl_462 (impl)
macro_rules! Depcrate_schema_schemaimpl_462 {
() => {
// Module: crate::schema::schema
// Provides: {"impl_462"}
// Dependencies: {}
# [graphql_object] # [graphql (name = "__EnumValue" , internal)] impl EnumValue { fn name (& self) -> & ArcStr { & self . name } # [graphql (name = "description")] fn description_ (& self) -> Option < & ArcStr > { self . description . as_ref () } fn is_deprecated (& self) -> bool { self . deprecation_status . is_deprecated () } fn deprecation_reason (& self) -> Option < & ArcStr > { self . deprecation_status . reason () } }
};
}
