// Generated macro for wrap_call_impl_with_call_once_impl (function)
macro_rules! Depcrate_render_fixturewrap_call_impl_with_call_once_impl {
() => {
// Module: crate::render::fixture
// Provides: {"wrap_call_impl_with_call_once_impl"}
// Dependencies: {}
fn wrap_call_impl_with_call_once_impl (call_impl : TokenStream , rt : & ReturnType) -> TokenStream { let std = std_path () ; match rt { syn :: ReturnType :: Type (_ , t) => parse_quote ! { static CELL : # std :: sync :: OnceLock <# t > = # std :: sync :: OnceLock :: new () ; CELL . get_or_init (|| # call_impl) } , _ => parse_quote ! { static CELL : # std :: sync :: Once = # std :: sync :: Once :: new () ; CELL . call_once (|| # call_impl) ; } , } }
};
}
