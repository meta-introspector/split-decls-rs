// Generated macro for impl_88 (impl)
macro_rules! Depcrate_providerimpl_88 {
() => {
// Module: crate::provider
// Provides: {"impl_88"}
// Dependencies: {}
impl < V > PluralElementsPackedCow < '_ , V > where V : VarULE + ? Sized , { # [doc = " Returns the value for the given [`PluralOperands`] and [`PluralRules`]."] pub fn get < 'a > (& 'a self , op : PluralOperands , rules : & PluralRules) -> & 'a V { self . elements . get (op , rules) . 1 } }
};
}
