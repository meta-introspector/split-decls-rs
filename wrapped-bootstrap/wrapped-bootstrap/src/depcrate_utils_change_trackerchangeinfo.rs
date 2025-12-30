// Generated macro for ChangeInfo (struct)
macro_rules! Depcrate_utils_change_trackerChangeInfo {
() => {
// Module: crate::utils::change_tracker
// Provides: {"ChangeInfo"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct ChangeInfo { # [doc = " Represents the ID of PR caused major change on bootstrap."] pub change_id : usize , pub severity : ChangeSeverity , # [doc = " Provides a short summary of the change that will guide developers"] # [doc = " on \"how to handle/behave\" in response to the changes."] pub summary : & 'static str , }
};
}
