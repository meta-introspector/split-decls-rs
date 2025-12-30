// Generated macro for SchemaBuilder (struct)
macro_rules! Depcrate_schemaSchemaBuilder {
() => {
// Module: crate::schema
// Provides: {"SchemaBuilder"}
// Dependencies: {}
# [doc = " Schema builder"] pub struct SchemaBuilder < Query , Mutation , Subscription > { validation_mode : ValidationMode , query : QueryRoot < Query > , mutation : Mutation , subscription : Subscription , registry : Registry , data : Data , complexity : Option < usize > , depth : Option < usize > , recursive_depth : usize , max_directives : Option < usize > , extensions : Vec < Box < dyn ExtensionFactory > > , custom_directives : HashMap < String , Box < dyn CustomDirectiveFactory > > , }
};
}
