// Generated macro for handling_magic_conversion_code (function)
macro_rules! Depcrate_render_injecthandling_magic_conversion_code {
() => {
// Module: crate::render::inject
// Provides: {"handling_magic_conversion_code"}
// Dependencies: {}
fn handling_magic_conversion_code (fixture : Cow < Expr > , arg_type : & Type) -> Expr { let rstest_path = crate_name () ; parse_quote ! { { use # rstest_path :: magic_conversion ::*; (&&& Magic ::<# arg_type > (core :: marker :: PhantomData)) . magic_conversion (# fixture) } } }
};
}
