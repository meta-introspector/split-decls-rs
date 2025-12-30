// Generated macro for resolve_arguments (function)
macro_rules! Depcrate_render_injectresolve_arguments {
() => {
// Module: crate::render::inject
// Provides: {"resolve_arguments"}
// Dependencies: {}
pub (crate) fn resolve_arguments < 'a > (args : impl Iterator < Item = & 'a FnArg > , resolver : & impl Resolver , generic_types : & [Ident] ,) -> TokenStream { let define_vars = args . map (| arg | ArgumentResolver :: new (resolver , generic_types) . resolve (arg)) ; quote ! { # (# define_vars) * } }
};
}
