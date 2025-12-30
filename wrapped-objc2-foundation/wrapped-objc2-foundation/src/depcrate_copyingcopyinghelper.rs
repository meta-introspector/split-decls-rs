// Generated macro for CopyingHelper (trait)
macro_rules! Depcrate_copyingCopyingHelper {
() => {
// Module: crate::copying
// Provides: {"CopyingHelper"}
// Dependencies: {}
# [doc = " A helper type for implementing [`NSCopying`]."] # [doc = ""] # [doc = " `NSCopying` and `NSMutableCopying` do not in their signatures describe the"] # [doc = " result type from the copying operation. This is problematic, as it means"] # [doc = " that using them ends up falling back to [`AnyObject`], which makes copying"] # [doc = " much less useful and ergonomic."] # [doc = ""] # [doc = " To properly describe this, we need an associated type which describes the"] # [doc = " actual result type from a copy. The associated type can't be present"] # [doc = " directly on the protocol traits themselves, however, since we want to use"] # [doc = " them as e.g. `ProtocolObject<dyn NSCopying>`, so we introduce this helper"] # [doc = " trait instead. See [`MutableCopyingHelper`] for the mutable variant."] # [doc = ""] # [doc = " We might be able to get rid of this hack once [associated type defaults]"] # [doc = " are stabilized."] # [doc = ""] # [doc = " [`AnyObject`]: objc2::runtime::AnyObject"] # [doc = " [associated type defaults]: https://github.com/rust-lang/rust/issues/29661"] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The [`Result`] type must be correct."] # [doc = ""] # [doc = " [`Result`]: Self::Result"] pub unsafe trait CopyingHelper : Message { # [doc = " The immutable counterpart of the type, or `Self` if the type has no"] # [doc = " immutable counterpart."] # [doc = ""] # [doc = " The implementation for `NSString` has itself (`NSString`) here, while"] # [doc = " `NSMutableString` instead has `NSString`."] type Result : Message ; }
};
}
