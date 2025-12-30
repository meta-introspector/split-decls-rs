// Generated macro for CFCopyLocalizedString (function)
macro_rules! Depcrate_bundleCFCopyLocalizedString {
() => {
// Module: crate::bundle
// Provides: {"CFCopyLocalizedString"}
// Dependencies: {}
# [allow (unused)] # [inline (always)] pub unsafe fn CFCopyLocalizedString (key : CFStringRef , comment : CFStringRef) -> CFStringRef { CFBundleCopyLocalizedString (CFBundleGetMainBundle () , key , key , std :: ptr :: null ()) }
};
}
