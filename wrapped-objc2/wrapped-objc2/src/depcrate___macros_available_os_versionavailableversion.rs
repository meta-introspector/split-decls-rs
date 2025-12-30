// Generated macro for AvailableVersion (struct)
macro_rules! Depcrate___macros_available_os_versionAvailableVersion {
() => {
// Module: crate::__macros::available::os_version
// Provides: {"AvailableVersion"}
// Dependencies: {}
# [doc = " The combined availability."] # [doc = ""] # [doc = " This generally works closely together with the `available!` macro to make"] # [doc = " syntax checking inside that easier."] # [derive (Clone , Copy , Debug)] pub struct AvailableVersion { pub macos : OSVersion , pub ios : OSVersion , pub tvos : OSVersion , pub watchos : OSVersion , pub visionos : OSVersion , # [doc (hidden)] pub __others : OSVersion , }
};
}
