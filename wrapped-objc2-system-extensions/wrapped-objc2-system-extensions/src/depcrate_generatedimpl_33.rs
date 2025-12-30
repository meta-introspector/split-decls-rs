// Generated macro for impl_33 (impl)
macro_rules! Depcrate_generatedimpl_33 {
() => {
// Module: crate::generated
// Provides: {"impl_33"}
// Dependencies: {}
impl OSSystemExtensionProperties { extern_methods ! (# [doc = " The file URL locating an indicating the extension bundle these properties"] # [doc = " were retreived from."] # [unsafe (method (URL))] # [unsafe (method_family = none)] pub unsafe fn URL (& self) -> Retained < NSURL >; # [doc = " The bundle identifier of the extension (CFBundleIdentifier)"] # [unsafe (method (bundleIdentifier))] # [unsafe (method_family = none)] pub unsafe fn bundleIdentifier (& self) -> Retained < NSString >; # [doc = " The bundle version of the extension (CFBundleVersion)"] # [unsafe (method (bundleVersion))] # [unsafe (method_family = none)] pub unsafe fn bundleVersion (& self) -> Retained < NSString >; # [doc = " The bundle short version string of the extension (CFBundleShortVersionString)"] # [unsafe (method (bundleShortVersion))] # [unsafe (method_family = none)] pub unsafe fn bundleShortVersion (& self) -> Retained < NSString >; # [doc = " Returns the enabled state of the extension"] # [unsafe (method (isEnabled))] # [unsafe (method_family = none)] pub unsafe fn isEnabled (& self) -> bool ; # [doc = " Returns whether an extension is waiting for user approval"] # [unsafe (method (isAwaitingUserApproval))] # [unsafe (method_family = none)] pub unsafe fn isAwaitingUserApproval (& self) -> bool ; # [doc = " Returns if an extension is being uninstalled"] # [unsafe (method (isUninstalling))] # [unsafe (method_family = none)] pub unsafe fn isUninstalling (& self) -> bool ;) ; }
};
}
