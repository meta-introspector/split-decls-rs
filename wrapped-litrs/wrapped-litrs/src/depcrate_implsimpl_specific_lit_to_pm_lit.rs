// Generated macro for impl_specific_lit_to_pm_lit (macro)
macro_rules! Depcrate_implsimpl_specific_lit_to_pm_lit {
() => {
// Module: crate::impls
// Provides: {"impl_specific_lit_to_pm_lit"}
// Dependencies: {}
macro_rules ! impl_specific_lit_to_pm_lit { ([$ ($ prefix : tt) *] => $ ty : ident , $ variant : ident , $ kind : ident) => { impl < B : crate :: Buffer > From < crate ::$ ty < B >> for $ ($ prefix) * Literal { fn from (l : crate ::$ ty < B >) -> Self { l . raw_input () . parse () . unwrap_or_else (| e | { panic ! ("failed to parse `{}` as `{}`: {}" , l . raw_input () , std :: any :: type_name ::< Self > () , e ,) }) } } } ; }
};
}
