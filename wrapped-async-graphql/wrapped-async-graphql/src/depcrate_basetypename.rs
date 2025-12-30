// Generated macro for TypeName (trait)
macro_rules! Depcrate_baseTypeName {
() => {
// Module: crate::base
// Provides: {"TypeName"}
// Dependencies: {}
# [doc = " Used to specify the GraphQL Type name."] pub trait TypeName : Send + Sync { # [doc = " Returns a GraphQL type name."] fn type_name () -> Cow < 'static , str > ; }
};
}
