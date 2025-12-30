// Generated macro for SchemaInner (struct)
macro_rules! Depcrate_schemaSchemaInner {
() => {
// Module: crate::schema
// Provides: {"SchemaInner"}
// Dependencies: {}
# [doc (hidden)] pub struct SchemaInner < Query , Mutation , Subscription > { pub (crate) validation_mode : ValidationMode , pub (crate) query : QueryRoot < Query > , pub (crate) mutation : Mutation , pub (crate) subscription : Subscription , pub (crate) complexity : Option < usize > , pub (crate) depth : Option < usize > , pub (crate) recursive_depth : usize , pub (crate) max_directives : Option < usize > , pub (crate) extensions : Vec < Box < dyn ExtensionFactory > > , pub (crate) env : SchemaEnv , }
};
}
