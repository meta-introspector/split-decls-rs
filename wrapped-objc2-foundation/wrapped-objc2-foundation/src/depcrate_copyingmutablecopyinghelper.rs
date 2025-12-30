// Generated macro for MutableCopyingHelper (trait)
macro_rules! Depcrate_copyingMutableCopyingHelper {
() => {
// Module: crate::copying
// Provides: {"MutableCopyingHelper"}
// Dependencies: {}
# [doc = " A helper type for implementing [`NSMutableCopying`]."] # [doc = ""] # [doc = " See [`CopyingHelper`] for the immutable variant, and more details in"] # [doc = " general. These traits are split to allow implementing"] # [doc = " `MutableCopyingHelper` only when the mutable class is available."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The [`Result`] type must be correct."] # [doc = ""] # [doc = " [`Result`]: Self::Result"] pub unsafe trait MutableCopyingHelper : Message { # [doc = " The mutable counterpart of the type, or `Self` if the type has no"] # [doc = " mutable counterpart."] # [doc = ""] # [doc = " The implementation for `NSString` has `NSMutableString` here, while"] # [doc = " `NSMutableString` has itself (`NSMutableString`)."] type Result : Message ; }
};
}
