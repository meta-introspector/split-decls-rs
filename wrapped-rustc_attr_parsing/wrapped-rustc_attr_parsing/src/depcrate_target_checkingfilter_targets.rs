// Generated macro for filter_targets (function)
macro_rules! Depcrate_target_checkingfilter_targets {
() => {
// Module: crate::target_checking
// Provides: {"filter_targets"}
// Dependencies: {}
fn filter_targets (allowed_targets : & mut Vec < Target > , target_group : & 'static [Target] , target_group_name : & 'static str , target : Target , added_fake_targets : & mut Vec < & 'static str > ,) { if target_group . contains (& target) { return ; } if allowed_targets . iter () . filter (| at | target_group . contains (at)) . count () < 2 { return ; } allowed_targets . retain (| t | ! target_group . contains (t)) ; added_fake_targets . push (target_group_name) ; }
};
}
