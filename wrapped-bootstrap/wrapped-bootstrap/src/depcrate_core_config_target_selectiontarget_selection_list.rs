// Generated macro for target_selection_list (function)
macro_rules! Depcrate_core_config_target_selectiontarget_selection_list {
() => {
// Module: crate::core::config::target_selection
// Provides: {"target_selection_list"}
// Dependencies: {}
pub fn target_selection_list (s : & str) -> Result < TargetSelectionList , String > { Ok (TargetSelectionList (s . split (',') . filter (| s | ! s . is_empty ()) . map (TargetSelection :: from_user) . collect () ,)) }
};
}
