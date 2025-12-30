// Generated macro for impl_1034 (impl)
macro_rules! Depcrate_core_config_target_selectionimpl_1034 {
() => {
// Module: crate::core::config::target_selection
// Provides: {"impl_1034"}
// Dependencies: {}
impl SplitDebuginfo { # [doc = " Returns the default `-Csplit-debuginfo` value for the current target. See the comment for"] # [doc = " `rust.split-debuginfo` in `bootstrap.example.toml`."] pub fn default_for_platform (target : TargetSelection) -> Self { if target . contains ("apple") { SplitDebuginfo :: Unpacked } else if target . is_windows () { SplitDebuginfo :: Packed } else { SplitDebuginfo :: Off } } }
};
}
