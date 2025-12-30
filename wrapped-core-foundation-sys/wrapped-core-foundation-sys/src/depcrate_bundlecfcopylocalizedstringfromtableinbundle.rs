// Generated macro for CFCopyLocalizedStringFromTableInBundle (function)
macro_rules! Depcrate_bundleCFCopyLocalizedStringFromTableInBundle {
() => {
// Module: crate::bundle
// Provides: {"CFCopyLocalizedStringFromTableInBundle"}
// Dependencies: {}
# [allow (unused)] # [inline (always)] pub unsafe fn CFCopyLocalizedStringFromTableInBundle (key : CFStringRef , tbl : CFStringRef , bundle : CFBundleRef , comment : CFStringRef ,) -> CFStringRef { CFBundleCopyLocalizedString (bundle , key , key , tbl) }
};
}
