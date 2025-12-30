// Generated macro for add_suffix_to_snapshot_name (function)
macro_rules! Depcrate_runtimeadd_suffix_to_snapshot_name {
() => {
// Module: crate::runtime
// Provides: {"add_suffix_to_snapshot_name"}
// Dependencies: {}
# [doc = " If there is a suffix on the settings, append it to the snapshot name."] fn add_suffix_to_snapshot_name (name : Cow < '_ , str >) -> Cow < '_ , str > { Settings :: with (| settings | { settings . snapshot_suffix () . map (| suffix | Cow :: Owned (format ! ("{name}@{suffix}"))) . unwrap_or_else (| | name) }) }
};
}
