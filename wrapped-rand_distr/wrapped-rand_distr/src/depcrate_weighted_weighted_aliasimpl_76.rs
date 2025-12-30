// Generated macro for impl_76 (impl)
macro_rules! Depcrate_weighted_weighted_aliasimpl_76 {
() => {
// Module: crate::weighted::weighted_alias
// Provides: {"impl_76"}
// Dependencies: {}
impl < W : AliasableWeight > Distribution < usize > for WeightedAliasIndex < W > { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> usize { let candidate = rng . sample (self . uniform_index) ; if rng . sample (& self . uniform_within_weight_sum) < self . no_alias_odds [candidate as usize] { candidate as usize } else { self . aliases [candidate as usize] as usize } } }
};
}
