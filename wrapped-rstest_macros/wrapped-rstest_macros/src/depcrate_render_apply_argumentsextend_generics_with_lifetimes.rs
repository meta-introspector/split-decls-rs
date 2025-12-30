// Generated macro for extend_generics_with_lifetimes (function)
macro_rules! Depcrate_render_apply_argumentsextend_generics_with_lifetimes {
() => {
// Module: crate::render::apply_arguments
// Provides: {"extend_generics_with_lifetimes"}
// Dependencies: {}
fn extend_generics_with_lifetimes < 'a , 'b > (generics : impl Iterator < Item = & 'a syn :: GenericParam > , lifetimes : impl Iterator < Item = & 'b syn :: Lifetime > ,) -> Generics { let all = lifetimes . map (| lt | lt as & dyn ToTokens) . chain (generics . map (| gp | gp as & dyn ToTokens)) ; parse_quote ! { <# (# all) ,*> } }
};
}
