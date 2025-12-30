// Generated macro for Pointer (trait)
macro_rules! Depcrate_extPointer {
() => {
// Module: crate::ext
// Provides: {"Pointer"}
// Dependencies: {}
# [doc = " A trait for adding some helper routines to pointers."] pub (crate) trait Pointer { # [doc = " Returns the distance, in units of `T`, between `self` and `origin`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Same as `ptr::offset_from` in addition to `self >= origin`."] unsafe fn distance (self , origin : Self) -> usize ; # [doc = " Casts this pointer to `usize`."] # [doc = ""] # [doc = " Callers should not convert the `usize` back to a pointer if at all"] # [doc = " possible. (And if you believe it's necessary, open an issue to discuss"] # [doc = " why. Otherwise, it has the potential to violate pointer provenance.)"] # [doc = " The purpose of this function is just to be able to do arithmetic, i.e.,"] # [doc = " computing offsets or alignments."] fn as_usize (self) -> usize ; }
};
}
