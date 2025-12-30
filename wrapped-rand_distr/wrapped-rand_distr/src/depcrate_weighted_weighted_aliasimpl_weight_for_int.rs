// Generated macro for impl_weight_for_int (macro)
macro_rules! Depcrate_weighted_weighted_aliasimpl_weight_for_int {
() => {
// Module: crate::weighted::weighted_alias
// Provides: {"impl_weight_for_int"}
// Dependencies: {}
macro_rules ! impl_weight_for_int { ($ T : ident) => { impl AliasableWeight for $ T { const MAX : Self = $ T :: MAX ; const ZERO : Self = 0 ; fn try_from_u32_lossy (n : u32) -> Option < Self > { let n_converted = n as Self ; if n_converted >= Self :: ZERO && n_converted as u32 == n { Some (n_converted) } else { None } } } } ; }
};
}
