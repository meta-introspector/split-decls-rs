// Generated macro for CFCopyLocalizedStringWithDefaultValue (function)
macro_rules! Depcrate_bundleCFCopyLocalizedStringWithDefaultValue {
() => {
// Module: crate::bundle
// Provides: {"CFCopyLocalizedStringWithDefaultValue"}
// Dependencies: {}
# [allow (unused)] # [inline (always)] pub unsafe fn CFCopyLocalizedStringWithDefaultValue (key : CFStringRef , tbl : CFStringRef , bundle : CFBundleRef , value : CFStringRef , comment : CFStringRef ,) -> CFStringRef { CFBundleCopyLocalizedString (bundle , key , value , tbl) }
};
}
