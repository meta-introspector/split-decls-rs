// Generated macro for SchemaBuilder (struct)
macro_rules! Depcrate_dynamic_schemaSchemaBuilder {
() => {
// Module: crate::dynamic::schema
// Provides: {"SchemaBuilder"}
// Dependencies: {}
# [doc = " Dynamic schema builder"] pub struct SchemaBuilder { query_type : String , mutation_type : Option < String > , subscription_type : Option < String > , types : IndexMap < String , Type > , data : Data , extensions : Vec < Box < dyn ExtensionFactory > > , validation_mode : ValidationMode , recursive_depth : usize , max_directives : Option < usize > , complexity : Option < usize > , depth : Option < usize > , enable_suggestions : bool , introspection_mode : IntrospectionMode , enable_federation : bool , entity_resolver : Option < BoxResolverFn > , }
};
}
