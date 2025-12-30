// Generated macro for impl_1793 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1793 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1793"}
// Dependencies: {}
impl < S , Selection > SelectDsl < Selection > for Alias < S > where Selection : Expression , Self : AsQuery , < Self as AsQuery > :: Query : SelectDsl < Selection > , { type Output = dsl :: Select < < Self as AsQuery > :: Query , Selection > ; fn select (self , selection : Selection) -> Self :: Output { self . as_query () . select (selection) } }
};
}
