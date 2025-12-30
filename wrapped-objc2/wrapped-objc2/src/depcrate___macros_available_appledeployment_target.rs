// Generated macro for DEPLOYMENT_TARGET (const)
macro_rules! Depcrate___macros_available_appleDEPLOYMENT_TARGET {
() => {
// Module: crate::__macros::available::apple
// Provides: {"DEPLOYMENT_TARGET"}
// Dependencies: {}
# [doc = " The deployment target for the current OS."] pub (crate) const DEPLOYMENT_TARGET : OSVersion = { # [cfg (target_os = "macos")] let var = option_env ! ("MACOSX_DEPLOYMENT_TARGET") ; # [cfg (target_os = "ios")] let var = option_env ! ("IPHONEOS_DEPLOYMENT_TARGET") ; # [cfg (target_os = "tvos")] let var = option_env ! ("TVOS_DEPLOYMENT_TARGET") ; # [cfg (target_os = "watchos")] let var = option_env ! ("WATCHOS_DEPLOYMENT_TARGET") ; # [cfg (target_os = "visionos")] let var = option_env ! ("XROS_DEPLOYMENT_TARGET") ; if let Some (var) = var { OSVersion :: from_str (var) } else { # [allow (clippy :: if_same_then_else)] let os_min = if cfg ! (target_os = "macos") { (10 , 12 , 0) } else if cfg ! (target_os = "ios") { (10 , 0 , 0) } else if cfg ! (target_os = "tvos") { (10 , 0 , 0) } else if cfg ! (target_os = "watchos") { (5 , 0 , 0) } else if cfg ! (target_os = "visionos") { (1 , 0 , 0) } else { panic ! ("unknown Apple OS") } ; # [allow (clippy :: if_same_then_else)] let min = if cfg ! (all (target_os = "macos" , target_arch = "aarch64")) { (11 , 0 , 0) } else if cfg ! (all (target_os = "ios" , target_arch = "aarch64" , target_abi_macabi)) { (14 , 0 , 0) } else if cfg ! (all (target_os = "ios" , target_arch = "aarch64" , target_simulator)) { (14 , 0 , 0) } else if cfg ! (all (target_os = "tvos" , target_arch = "aarch64")) { (14 , 0 , 0) } else if cfg ! (all (target_os = "watchos" , target_arch = "aarch64")) { (7 , 0 , 0) } else { os_min } ; OSVersion { major : min . 0 , minor : min . 1 , patch : min . 2 , } } } ;
};
}
