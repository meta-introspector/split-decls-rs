// Generated macro for Tag (trait)
macro_rules! Depcrate_tagged_ptrTag {
() => {
// Module: crate::tagged_ptr
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " This describes tags that the [`TaggedRef`] struct can hold."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - The [`BITS`] constant must be correct."] # [doc = " - No more than [`BITS`] least-significant bits may be set in the returned usize."] # [doc = " - [`Eq`] and [`Hash`] must be implementable with the returned `usize` from `into_usize`."] # [doc = ""] # [doc = " [`BITS`]: Tag::BITS"] pub unsafe trait Tag : Copy { # [doc = " Number of least-significant bits in the return value of [`into_usize`]"] # [doc = " which may be non-zero. In other words this is the bit width of the"] # [doc = " value."] # [doc = ""] # [doc = " [`into_usize`]: Tag::into_usize"] const BITS : u32 ; # [doc = " Turns this tag into an integer."] # [doc = ""] # [doc = " The inverse of this function is [`from_usize`]."] # [doc = ""] # [doc = " This function guarantees that only the least-significant [`Self::BITS`]"] # [doc = " bits can be non-zero."] # [doc = ""] # [doc = " [`from_usize`]: Tag::from_usize"] # [doc = " [`Self::BITS`]: Tag::BITS"] fn into_usize (self) -> usize ; # [doc = " Re-creates the tag from the integer returned by [`into_usize`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The passed `tag` must be returned from [`into_usize`]."] # [doc = ""] # [doc = " [`into_usize`]: Tag::into_usize"] unsafe fn from_usize (tag : usize) -> Self ; }
};
}
