// Generated macro for ArgumentResolver (struct)
macro_rules! Depcrate_render_injectArgumentResolver {
() => {
// Module: crate::render::inject
// Provides: {"ArgumentResolver"}
// Dependencies: {}
struct ArgumentResolver < 'resolver , 'idents , 'f , R > where R : Resolver + 'resolver , { resolver : & 'resolver R , generic_types_names : & 'idents [Ident] , magic_conversion : & 'f dyn Fn (Cow < Expr > , & Type) -> Expr , }
};
}
