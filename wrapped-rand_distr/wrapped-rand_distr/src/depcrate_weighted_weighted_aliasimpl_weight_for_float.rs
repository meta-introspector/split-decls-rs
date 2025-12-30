// Generated macro for impl_weight_for_float (macro)
macro_rules! Depcrate_weighted_weighted_aliasimpl_weight_for_float {
() => {
// Module: crate::weighted::weighted_alias
// Provides: {"impl_weight_for_float"}
// Dependencies: {}
macro_rules ! impl_weight_for_float { ($ T : ident) => { impl AliasableWeight for $ T { const MAX : Self = $ T :: MAX ; const ZERO : Self = 0.0 ; fn try_from_u32_lossy (n : u32) -> Option < Self > { Some (n as $ T) } fn sum (values : & [Self]) -> Self { pairwise_sum (values) } } } ; }
};
}
