// Generated macro for DecoratableTarget (trait)
macro_rules! Depcrate_query_builder_upsert_on_conflict_target_decorationsDecoratableTarget {
() => {
// Module: crate::query_builder::upsert::on_conflict_target_decorations
// Provides: {"DecoratableTarget"}
// Dependencies: {}
# [doc = " Interface to add information to conflict targets."] # [doc = " Designed to be open for further additions to conflict targets like constraints"] pub trait DecoratableTarget < P > { # [doc = " Output type of filter_target operation"] type FilterOutput ; # [doc = " equivalent to filter of FilterDsl but aimed at conflict targets"] fn filter_target (self , predicate : P) -> Self :: FilterOutput ; }
};
}
