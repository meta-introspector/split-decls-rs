// Generated macro for assert_dep_graph (function)
macro_rules! Depcrate_assert_dep_graphassert_dep_graph {
() => {
// Module: crate::assert_dep_graph
// Provides: {"assert_dep_graph"}
// Dependencies: {}
# [allow (missing_docs)] pub (crate) fn assert_dep_graph (tcx : TyCtxt < '_ >) { tcx . dep_graph . with_ignore (| | { if tcx . sess . opts . unstable_opts . dump_dep_graph { tcx . dep_graph . with_query (dump_graph) ; } if ! tcx . sess . opts . unstable_opts . query_dep_graph { return ; } if ! tcx . features () . rustc_attrs () { return ; } let (if_this_changed , then_this_would_need) = { let mut visitor = IfThisChanged { tcx , if_this_changed : vec ! [] , then_this_would_need : vec ! [] } ; visitor . process_attrs (CRATE_DEF_ID) ; tcx . hir_visit_all_item_likes_in_crate (& mut visitor) ; (visitor . if_this_changed , visitor . then_this_would_need) } ; if ! if_this_changed . is_empty () || ! then_this_would_need . is_empty () { assert ! (tcx . sess . opts . unstable_opts . query_dep_graph , "cannot use the `#[{}]` or `#[{}]` annotations \
                    without supplying `-Z query-dep-graph`" , sym :: rustc_if_this_changed , sym :: rustc_then_this_would_need) ; } check_paths (tcx , & if_this_changed , & then_this_would_need) ; }) }
};
}
