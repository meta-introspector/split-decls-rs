// Generated macro for impl_77 (impl)
macro_rules! Depcrate_weighted_weighted_aliasimpl_77 {
() => {
// Module: crate::weighted::weighted_alias
// Provides: {"impl_77"}
// Dependencies: {}
impl < W : AliasableWeight > fmt :: Debug for WeightedAliasIndex < W > where W : fmt :: Debug , Uniform < W > : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("WeightedAliasIndex") . field ("aliases" , & self . aliases) . field ("no_alias_odds" , & self . no_alias_odds) . field ("uniform_index" , & self . uniform_index) . field ("uniform_within_weight_sum" , & self . uniform_within_weight_sum) . finish () } }
};
}
