// Generated macro for apple_os (function)
macro_rules! Depcrate_targetsapple_os {
() => {
// Module: crate::targets
// Provides: {"apple_os"}
// Dependencies: {}
# [doc = " Get the target OS on Apple operating systems."] # [must_use] pub fn apple_os () -> & 'static str { if target () . contains ("darwin") { "macos" } else if target () . contains ("ios") { "ios" } else if target () . contains ("tvos") { "tvos" } else if target () . contains ("watchos") { "watchos" } else if target () . contains ("visionos") { "visionos" } else { panic ! ("not an Apple OS") } }
};
}
