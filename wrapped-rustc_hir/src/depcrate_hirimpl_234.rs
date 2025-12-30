// Generated macro for impl_234 (impl)
macro_rules! Depcrate_hirimpl_234 {
() => {
// Module: crate::hir
// Provides: {"impl_234"}
// Dependencies: {}
impl < 'hir > GenericParam < 'hir > { # [doc = " Synthetic type-parameters are inserted after normal ones."] # [doc = " In order for normal parameters to be able to refer to synthetic ones,"] # [doc = " scans them first."] pub fn is_impl_trait (& self) -> bool { matches ! (self . kind , GenericParamKind :: Type { synthetic : true , .. }) } # [doc = " This can happen for `async fn`, e.g. `async fn f<'_>(&'_ self)`."] # [doc = ""] # [doc = " See `lifetime_to_generic_param` in `rustc_ast_lowering` for more information."] pub fn is_elided_lifetime (& self) -> bool { matches ! (self . kind , GenericParamKind :: Lifetime { kind : LifetimeParamKind :: Elided (_) }) } }
};
}
