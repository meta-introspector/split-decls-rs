// Generated macro for impl_78 (impl)
macro_rules! Depcrate_weighted_weighted_aliasimpl_78 {
() => {
// Module: crate::weighted::weighted_alias
// Provides: {"impl_78"}
// Dependencies: {}
impl < W : AliasableWeight > Clone for WeightedAliasIndex < W > where Uniform < W > : Clone , { fn clone (& self) -> Self { Self { aliases : self . aliases . clone () , no_alias_odds : self . no_alias_odds . clone () , uniform_index : self . uniform_index , uniform_within_weight_sum : self . uniform_within_weight_sum . clone () , weight_sum : self . weight_sum , } } }
};
}
