// Generated macro for macro_288 (macro)
macro_rules! Depcrate_arbitrary__alloc_charmacro_288 {
() => {
// Module: crate::arbitrary::_alloc::char
// Provides: {"macro_288"}
// Dependencies: {}
arbitrary ! (ParseCharError , IndFlatten < Mapped < bool , Just < Self >>>; any ::< bool > () . prop_ind_flat_map (| is_two | Just ((if is_two { "__" } else { "" }) . parse ::< char > () . unwrap_err ()))) ;
};
}
