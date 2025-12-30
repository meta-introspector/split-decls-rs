// Generated macro for ArrayLike (trait)
macro_rules! Depcrate_read_utilArrayLike {
() => {
// Module: crate::read::util
// Provides: {"ArrayLike"}
// Dependencies: {}
# [doc = " Marker trait for types that can be used as backing storage when a growable array type is needed."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside this crate."] pub trait ArrayLike : Sealed { # [doc = " Type of the elements being stored."] type Item ; # [doc (hidden)] fn as_slice (storage : & Self :: Storage) -> & [MaybeUninit < Self :: Item >] ; # [doc (hidden)] fn as_mut_slice (storage : & mut Self :: Storage) -> & mut [MaybeUninit < Self :: Item >] ; }
};
}
