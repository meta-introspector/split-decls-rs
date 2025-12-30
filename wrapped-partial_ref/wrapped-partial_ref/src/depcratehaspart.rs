// Generated macro for HasPart (trait)
macro_rules! DepcrateHasPart {
() => {
// Module: crate
// Provides: {"HasPart"}
// Dependencies: {}
# [doc = " Implemented when a reference target has a part."] # [doc = ""] # [doc = " This trait provides methods for unchecked access to a part of a reference target."] # [doc = " Implementations for this are automatically created when deriving PartialRefTarget."] pub trait HasPart < SomePart : Part > : PartialRefTarget { # [doc = " Given a constant pointer to a target, produce a constant pointer to a part of it."] # [doc = ""] # [doc = " # Safety"] # [doc = " Implementations may construct a temporary reference to ptr, which thus must be valid."] unsafe fn part_ptr (ptr : * const Self :: RawTarget) -> < SomePart :: PartType as PartType > :: Ptr ; # [doc = " Given a mutable pointer to a target, produce a mutable pointer to a part of it."] # [doc = ""] # [doc = " # Safety"] # [doc = " Implementations may construct a temporary reference to ptr, which thus must be valid."] unsafe fn part_ptr_mut (ptr : * mut Self :: RawTarget) -> < SomePart :: PartType as PartType > :: PtrMut ; }
};
}
