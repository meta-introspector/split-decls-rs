// Generated macro for other_236 (other)
macro_rules! Depcrate_errorother_236 {
() => {
// Module: crate::error
// Provides: {"other_236"}
// Dependencies: {}
unsafe extern "C" { pub static kCFErrorDomainPOSIX : CFStringRef ; pub static kCFErrorDomainOSStatus : CFStringRef ; pub static kCFErrorDomainMach : CFStringRef ; pub static kCFErrorDomainCocoa : CFStringRef ; pub static kCFErrorLocalizedDescriptionKey : CFStringRef ; pub static kCFErrorLocalizedFailureReasonKey : CFStringRef ; pub static kCFErrorLocalizedRecoverySuggestionKey : CFStringRef ; pub static kCFErrorDescriptionKey : CFStringRef ; pub static kCFErrorUnderlyingErrorKey : CFStringRef ; pub static kCFErrorURLKey : CFStringRef ; pub static kCFErrorFilePathKey : CFStringRef ; pub fn CFErrorCreate (allocator : CFAllocatorRef , domain : CFErrorDomain , code : CFIndex , userInfo : CFDictionaryRef ,) -> CFErrorRef ; pub fn CFErrorGetDomain (err : CFErrorRef) -> CFStringRef ; pub fn CFErrorGetCode (err : CFErrorRef) -> CFIndex ; pub fn CFErrorCopyUserInfo (err : CFErrorRef) -> CFDictionaryRef ; pub fn CFErrorCopyDescription (err : CFErrorRef) -> CFStringRef ; pub fn CFErrorCopyFailureReason (err : CFErrorRef) -> CFStringRef ; pub fn CFErrorCopyRecoverySuggestion (err : CFErrorRef) -> CFStringRef ; pub fn CFErrorGetTypeID () -> CFTypeID ; }
};
}
