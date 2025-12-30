// Generated macro for find_recent_config_change_ids (function)
macro_rules! Depcrate_utils_change_trackerfind_recent_config_change_ids {
() => {
// Module: crate::utils::change_tracker
// Provides: {"find_recent_config_change_ids"}
// Dependencies: {}
pub fn find_recent_config_change_ids (current_id : usize) -> & 'static [ChangeInfo] { if let Some (index) = CONFIG_CHANGE_HISTORY . iter () . position (| config | config . change_id == current_id) { & CONFIG_CHANGE_HISTORY [index + 1 ..] } else { if let Some (config) = CONFIG_CHANGE_HISTORY . iter () . max_by_key (| config | config . change_id) && current_id > config . change_id { return & [] ; } CONFIG_CHANGE_HISTORY } }
};
}
