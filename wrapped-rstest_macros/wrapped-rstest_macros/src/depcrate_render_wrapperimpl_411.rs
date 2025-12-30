// Generated macro for impl_411 (impl)
macro_rules! Depcrate_render_wrapperimpl_411 {
() => {
// Module: crate::render::wrapper
// Provides: {"impl_411"}
// Dependencies: {}
impl < T : ToTokens > WrapByModule for T { fn wrap_by_mod (& self , mod_name : & Ident) -> TokenStream { quote ! { mod # mod_name { use super ::*; # self } } } }
};
}
