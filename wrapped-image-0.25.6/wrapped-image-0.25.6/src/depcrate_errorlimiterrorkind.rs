// Generated macro for LimitErrorKind (enum)
macro_rules! Depcrate_errorLimitErrorKind {
() => {
// Module: crate::error
// Provides: {"LimitErrorKind"}
// Dependencies: {}
# [doc = " Indicates the limit that prevented an operation from completing."] # [doc = ""] # [doc = " Note that this enumeration is not exhaustive and may in the future be extended to provide more"] # [doc = " detailed information or to incorporate other resources types."] # [derive (Clone , Debug , Hash , PartialEq , Eq)] # [non_exhaustive] # [allow (missing_copy_implementations)] pub enum LimitErrorKind { # [doc = " The resulting image exceed dimension limits in either direction."] DimensionError , # [doc = " The operation would have performed an allocation larger than allowed."] InsufficientMemory , # [doc = " The specified strict limits are not supported for this operation"] Unsupported { # [doc = " The given limits"] limits : crate :: Limits , # [doc = " The supported strict limits"] supported : crate :: LimitSupport , } , }
};
}
