// Generated macro for ResolverContext (struct)
macro_rules! Depcrate_dynamic_fieldResolverContext {
() => {
// Module: crate::dynamic::field
// Provides: {"ResolverContext"}
// Dependencies: {}
# [doc = " A context for resolver function"] pub struct ResolverContext < 'a > { # [doc = " GraphQL context"] pub ctx : & 'a Context < 'a > , # [doc = " Field arguments"] pub args : ObjectAccessor < 'a > , # [doc = " Parent value"] pub parent_value : & 'a FieldValue < 'a > , }
};
}
