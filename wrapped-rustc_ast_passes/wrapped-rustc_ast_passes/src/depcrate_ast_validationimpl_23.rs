// Generated macro for impl_23 (impl)
macro_rules! Depcrate_ast_validationimpl_23 {
() => {
// Module: crate::ast_validation
// Provides: {"impl_23"}
// Dependencies: {}
impl TraitOrTraitImpl { fn constness (& self) -> Option < Span > { match self { Self :: Trait { constness : Const :: Yes (span) , .. } | Self :: TraitImpl { constness : Const :: Yes (span) , .. } => Some (* span) , _ => None , } } }
};
}
