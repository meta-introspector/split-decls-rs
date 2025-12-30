// Generated macro for impl_463 (impl)
macro_rules! Depcrate_schema_schemaimpl_463 {
() => {
// Module: crate::schema::schema
// Provides: {"impl_463"}
// Dependencies: {}
# [graphql_object] # [graphql (name = "__Directive" , context = SchemaType < S >, scalar = S , internal ,)] impl < S : ScalarValue > DirectiveType < S > { fn name (& self) -> & ArcStr { & self . name } # [graphql (name = "description")] fn description_ (& self) -> Option < & ArcStr > { self . description . as_ref () } fn is_repeatable (& self) -> bool { self . is_repeatable } fn locations (& self) -> & [DirectiveLocation] { & self . locations } fn args (& self , # [graphql (default)] include_deprecated : bool) -> Vec < & Argument < S > > { self . arguments . iter () . filter (| a | include_deprecated || ! a . deprecation_status . is_deprecated ()) . collect () } # [graphql (deprecated = "Use `__Directive.locations` instead.")] fn on_operation (& self) -> bool { self . locations . contains (& DirectiveLocation :: Query) } # [graphql (deprecated = "Use `__Directive.locations` instead.")] fn on_fragment (& self) -> bool { self . locations . contains (& DirectiveLocation :: FragmentDefinition) || self . locations . contains (& DirectiveLocation :: InlineFragment) || self . locations . contains (& DirectiveLocation :: FragmentSpread) } # [graphql (deprecated = "Use `__Directive.locations` instead.")] fn on_field (& self) -> bool { self . locations . contains (& DirectiveLocation :: Field) } }
};
}
