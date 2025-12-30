// Generated macro for is_available (function)
macro_rules! Depcrate___macros_available_os_versionis_available {
() => {
// Module: crate::__macros::available::os_version
// Provides: {"is_available"}
// Dependencies: {}
# [inline] pub fn is_available (version : AvailableVersion) -> bool { let version = if cfg ! (target_os = "macos") { version . macos } else if cfg ! (target_os = "ios") { version . ios } else if cfg ! (target_os = "tvos") { version . tvos } else if cfg ! (target_os = "watchos") { version . watchos } else if cfg ! (target_os = "visionos") { version . visionos } else { version . __others } ; if version == OSVersion :: MAX { return false ; } # [cfg (target_vendor = "apple")] { if version <= super :: apple :: DEPLOYMENT_TARGET { return true ; } version <= super :: apple :: current_version () } # [cfg (not (target_vendor = "apple"))] return true ; }
};
}
