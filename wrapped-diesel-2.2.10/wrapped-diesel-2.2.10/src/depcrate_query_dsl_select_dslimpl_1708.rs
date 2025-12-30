// Generated macro for impl_1708 (impl)
macro_rules! Depcrate_query_dsl_select_dslimpl_1708 {
() => {
// Module: crate::query_dsl::select_dsl
// Provides: {"impl_1708"}
// Dependencies: {}
impl < T , Selection > SelectDsl < Selection > for T where Selection : Expression , T : Table , T :: Query : SelectDsl < Selection > , { type Output = < T :: Query as SelectDsl < Selection > > :: Output ; fn select (self , selection : Selection) -> Self :: Output { self . as_query () . select (selection) } }
};
}
