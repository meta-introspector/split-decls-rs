// Generated macro for Keyed (struct)
macro_rules! Depcrate_future_future_groupKeyed {
() => {
// Module: crate::future::future_group
// Provides: {"Keyed"}
// Dependencies: {}
# [doc = " Iterate over items in the futures group with their associated keys."] # [derive (Debug)] # [pin_project :: pin_project] pub struct Keyed < F : Future > { # [pin] group : FutureGroup < F > , }
};
}
