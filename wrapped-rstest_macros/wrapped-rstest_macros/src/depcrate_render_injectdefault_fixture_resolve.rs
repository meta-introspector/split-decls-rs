// Generated macro for default_fixture_resolve (function)
macro_rules! Depcrate_render_injectdefault_fixture_resolve {
() => {
// Module: crate::render::inject
// Provides: {"default_fixture_resolve"}
// Dependencies: {}
fn default_fixture_resolve (ident : & Ident) -> Cow < '_ , Expr > { Cow :: Owned (parse_quote ! { # ident :: default () }) }
};
}
