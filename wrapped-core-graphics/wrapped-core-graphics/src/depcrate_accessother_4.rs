// Generated macro for other_4 (other)
macro_rules! Depcrate_accessother_4 {
() => {
// Module: crate::access
// Provides: {"other_4"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { fn CGRequestScreenCaptureAccess () -> bool ; fn CGPreflightScreenCaptureAccess () -> bool ; }
};
}
