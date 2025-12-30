// Generated macro for impl_1867 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1867 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1867"}
// Dependencies: {}
impl < DB : Backend > nodes :: MiddleFragment < DB > for OnKeyword { fn push_sql (& self , mut pass : AstPass < '_ , '_ , DB >) { pass . push_sql (" ON ") ; } }
};
}
