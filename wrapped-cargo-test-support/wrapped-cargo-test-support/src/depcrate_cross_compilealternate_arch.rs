// Generated macro for alternate_arch (function)
macro_rules! Depcrate_cross_compilealternate_arch {
() => {
// Module: crate::cross_compile
// Provides: {"alternate_arch"}
// Dependencies: {}
pub fn alternate_arch () -> & 'static str { if cfg ! (target_os = "macos") { "x86_64" } else { "x86" } }
};
}
