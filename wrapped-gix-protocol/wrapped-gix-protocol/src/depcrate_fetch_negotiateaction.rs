// Generated macro for Action (enum)
macro_rules! Depcrate_fetch_negotiateAction {
() => {
// Module: crate::fetch::negotiate
// Provides: {"Action"}
// Dependencies: {}
# [doc = " Determines what should be done after [preparing the commit-graph for negotiation](mark_complete_and_common_ref)."] # [must_use] # [derive (Debug , Clone)] pub enum Action { # [doc = " None of the remote refs moved compared to our last recorded state (via tracking refs), so there is nothing to do at all,"] # [doc = " not even a ref update."] NoChange , # [doc = " Don't negotiate, don't fetch the pack, skip right to updating the references."] # [doc = ""] # [doc = " This happens if we already have all local objects even though the server seems to have changed."] SkipToRefUpdate , # [doc = " We can't know for sure if fetching *is not* needed, so we go ahead and negotiate."] MustNegotiate { # [doc = " Each `ref_map.mapping` has a slot here which is `true` if we have the object the remote ref points to, locally."] remote_ref_target_known : Vec < bool > , } , }
};
}
