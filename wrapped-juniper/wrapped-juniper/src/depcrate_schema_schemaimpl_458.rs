// Generated macro for impl_458 (impl)
macro_rules! Depcrate_schema_schemaimpl_458 {
() => {
// Module: crate::schema::schema
// Provides: {"impl_458"}
// Dependencies: {}
# [graphql_object] # [graphql (name = "__Schema" context = SchemaType < S >, scalar = S , internal ,)] impl < S : ScalarValue > SchemaType < S > { fn description (& self) -> Option < & ArcStr > { self . description . as_ref () } fn types (& self) -> Vec < TypeType < '_ , S > > { self . type_list () . into_iter () . filter (| t | { t . to_concrete () . map (| t | { ! (t . name () . map (ArcStr :: as_str) == Some ("_EmptyMutation") || t . name () . map (ArcStr :: as_str) == Some ("_EmptySubscription")) }) . unwrap_or (false) }) . collect () } # [graphql (name = "queryType")] fn query_type_ (& self) -> TypeType < '_ , S > { self . query_type () } # [graphql (name = "mutationType")] fn mutation_type_ (& self) -> Option < TypeType < '_ , S > > { self . mutation_type () } # [graphql (name = "subscriptionType")] fn subscription_type_ (& self) -> Option < TypeType < '_ , S > > { self . subscription_type () } fn directives (& self) -> Vec < & DirectiveType < S > > { self . directive_list () } }
};
}
