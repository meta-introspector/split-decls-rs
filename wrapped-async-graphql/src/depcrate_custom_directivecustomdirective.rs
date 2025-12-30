// Generated macro for CustomDirective (trait)
macro_rules! Depcrate_custom_directiveCustomDirective {
() => {
// Module: crate::custom_directive
// Provides: {"CustomDirective"}
// Dependencies: {}
# [doc = " Represents a custom directive."] # [async_trait :: async_trait] # [allow (unused_variables)] pub trait CustomDirective : Sync + Send + 'static { # [doc = " Called at resolve field."] async fn resolve_field (& self , ctx : & Context < '_ > , resolve : ResolveFut < '_ > ,) -> ServerResult < Option < Value > > { resolve . await } }
};
}
