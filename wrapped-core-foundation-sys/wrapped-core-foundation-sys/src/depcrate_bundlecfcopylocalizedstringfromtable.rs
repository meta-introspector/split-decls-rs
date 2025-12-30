// Generated macro for CFCopyLocalizedStringFromTable (function)
macro_rules! Depcrate_bundleCFCopyLocalizedStringFromTable {
() => {
// Module: crate::bundle
// Provides: {"CFCopyLocalizedStringFromTable"}
// Dependencies: {}
# [allow (unused)] # [inline (always)] pub unsafe fn CFCopyLocalizedStringFromTable (key : CFStringRef , tbl : CFStringRef , comment : CFStringRef ,) -> CFStringRef { CFBundleCopyLocalizedString (CFBundleGetMainBundle () , key , key , tbl) }
};
}
