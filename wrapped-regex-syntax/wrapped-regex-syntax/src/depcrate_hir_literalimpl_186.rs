// Generated macro for impl_186 (impl)
macro_rules! Depcrate_hir_literalimpl_186 {
() => {
// Module: crate::hir::literal
// Provides: {"impl_186"}
// Dependencies: {}
impl ExtractKind { # [doc = " Returns true if this kind is the `Prefix` variant."] pub fn is_prefix (& self) -> bool { matches ! (* self , ExtractKind :: Prefix) } # [doc = " Returns true if this kind is the `Suffix` variant."] pub fn is_suffix (& self) -> bool { matches ! (* self , ExtractKind :: Suffix) } }
};
}
