// Generated macro for SkeletonError (enum)
macro_rules! Depcrate_provider_skeleton_errorSkeletonError {
() => {
// Module: crate::provider::skeleton::error
// Provides: {"SkeletonError"}
// Dependencies: {}
# [doc = " These strings follow the recommendations for the serde::de::Unexpected::Other type."] # [doc = " <https://docs.serde.rs/serde/de/enum.Unexpected.html#variant.Other>"] # [doc = ""] # [doc = " Serde will generate an error such as:"] # [doc = " \"invalid value: unclosed literal in pattern, expected a valid UTS 35 pattern string at line 1 column 12\""] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Display , Debug , Copy , Clone , PartialEq)] # [allow (missing_docs)] # [non_exhaustive] pub enum SkeletonError { # [displaydoc ("field too long in skeleton")] InvalidFieldLength , # [displaydoc ("duplicate field in skeleton")] DuplicateField , # [displaydoc ("symbol unknown {0} in skeleton")] SymbolUnknown (char) , # [displaydoc ("symbol invalid {0} in skeleton")] SymbolInvalid (u8) , # [displaydoc ("symbol unimplemented {0} in skeleton")] SymbolUnimplemented (char) , # [displaydoc ("unimplemented field {0} in skeleton")] UnimplementedField (char) , # [displaydoc ("skeleton has a variant subtag")] SkeletonHasVariant , # [displaydoc ("{0}")] Fields (fields :: Error) , }
};
}
