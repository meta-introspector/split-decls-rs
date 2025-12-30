// Generated macro for ChangeSeverity (enum)
macro_rules! Depcrate_utils_change_trackerChangeSeverity {
() => {
// Module: crate::utils::change_tracker
// Provides: {"ChangeSeverity"}
// Dependencies: {}
# [derive (Clone , Debug)] pub enum ChangeSeverity { # [doc = " Used when build configurations continue working as before."] Info , # [doc = " Used when the default value of an option changes, or support for an option is removed entirely,"] # [doc = " potentially requiring developers to update their build configurations."] Warning , }
};
}
