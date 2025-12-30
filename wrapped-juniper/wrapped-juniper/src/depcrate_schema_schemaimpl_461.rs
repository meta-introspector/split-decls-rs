// Generated macro for impl_461 (impl)
macro_rules! Depcrate_schema_schemaimpl_461 {
() => {
// Module: crate::schema::schema
// Provides: {"impl_461"}
// Dependencies: {}
# [graphql_object] # [graphql (name = "__InputValue" , context = SchemaType < S >, scalar = S , internal ,)] impl < S : ScalarValue > Argument < S > { fn name (& self) -> & ArcStr { & self . name } # [graphql (name = "description")] fn description_ (& self) -> Option < & ArcStr > { self . description . as_ref () } # [graphql (name = "type")] fn type_ < 's > (& self , context : & 's SchemaType < S >) -> TypeType < 's , S > { context . make_type (& self . arg_type) } # [graphql (name = "defaultValue")] fn default_value_ (& self) -> Option < String > { self . default_value . as_ref () . map (ToString :: to_string) } fn is_deprecated (& self) -> bool { self . deprecation_status . is_deprecated () } fn deprecation_reason (& self) -> Option < & ArcStr > { self . deprecation_status . reason () } }
};
}
