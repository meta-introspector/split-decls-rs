// Generated macro for PartialRefTarget (trait)
macro_rules! DepcratePartialRefTarget {
() => {
// Module: crate
// Provides: {"PartialRefTarget"}
// Dependencies: {}
# [doc = " A type that can be the target of partial references."] # [doc = ""] # [doc = " Implementations for this trait should be derived, see [`partial_ref_derive`]."] pub trait PartialRefTarget { # [doc = " A partial reference will be represented by a pointer to this associated type."] # [doc = ""] # [doc = " In most cases this is the implementing type itself. This is not a requirement though. In the"] # [doc = " future, some features will introduce PartialRefTargets that have a different associated"] # [doc = " `RawTarget` type."] type RawTarget : ? Sized ; }
};
}
