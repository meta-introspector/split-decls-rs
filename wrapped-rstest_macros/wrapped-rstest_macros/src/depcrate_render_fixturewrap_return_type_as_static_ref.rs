// Generated macro for wrap_return_type_as_static_ref (function)
macro_rules! Depcrate_render_fixturewrap_return_type_as_static_ref {
() => {
// Module: crate::render::fixture
// Provides: {"wrap_return_type_as_static_ref"}
// Dependencies: {}
fn wrap_return_type_as_static_ref (rt : ReturnType) -> ReturnType { match rt { syn :: ReturnType :: Type (_ , t) => parse_quote ! { -> &'static # t } , o => o , } }
};
}
