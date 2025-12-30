// Generated macro for LifetimeReturnCategoryErr (enum)
macro_rules! Depcrate_session_diagnosticsLifetimeReturnCategoryErr {
() => {
// Module: crate::session_diagnostics
// Provides: {"LifetimeReturnCategoryErr"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum LifetimeReturnCategoryErr < 'a > { # [label (borrowck_returned_lifetime_wrong)] WrongReturn { # [primary_span] span : Span , mir_def_name : & 'a str , outlived_fr_name : RegionName , fr_name : & 'a RegionName , } , # [label (borrowck_returned_lifetime_short)] ShortReturn { # [primary_span] span : Span , category_desc : & 'static str , free_region_name : & 'a RegionName , outlived_fr_name : RegionName , } , }
};
}
