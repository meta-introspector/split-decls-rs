// Generated macro for impl_460 (impl)
macro_rules! Depcrate_schema_schemaimpl_460 {
() => {
// Module: crate::schema::schema
// Provides: {"impl_460"}
// Dependencies: {}
# [graphql_object] # [graphql (name = "__Field" , context = SchemaType < S >, scalar = S , internal ,)] impl < S : ScalarValue > Field < S > { fn name (& self) -> & ArcStr { & self . name } # [graphql (name = "description")] fn description_ (& self) -> Option < & ArcStr > { self . description . as_ref () } fn args (& self , # [graphql (default)] include_deprecated : bool) -> Vec < & Argument < S > > { self . arguments . as_ref () . map_or_else (Vec :: new , | args | { args . iter () . filter (| a | include_deprecated || ! a . deprecation_status . is_deprecated ()) . collect () }) } # [graphql (name = "type")] fn type_ < 's > (& self , context : & 's SchemaType < S >) -> TypeType < 's , S > { context . make_type (& self . field_type) } fn is_deprecated (& self) -> bool { self . deprecation_status . is_deprecated () } fn deprecation_reason (& self) -> Option < & ArcStr > { self . deprecation_status . reason () } }
};
}
