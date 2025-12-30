// Generated macro for Storage (trait)
macro_rules! Depcrate_base_storageStorage {
() => {
// Module: crate::base::storage
// Provides: {"Storage"}
// Dependencies: {}
# [doc = " Trait shared by all matrix data storage that don’t contain any uninitialized elements."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Note that `Self` must always have a number of elements compatible with the matrix length (given"] # [doc = " by `R` and `C` if they are known at compile-time). For example, implementors of this trait"] # [doc = " should **not** allow the user to modify the size of the underlying buffer with safe methods"] # [doc = " (for example the `VecStorage::data_mut` method is unsafe because the user could change the"] # [doc = " vector's size so that it no longer contains enough elements: this will lead to UB."] pub unsafe trait Storage < T : Scalar , R : Dim , C : Dim = U1 > : RawStorage < T , R , C > { # [doc = " Builds a matrix data storage that does not contain any reference."] fn into_owned (self) -> Owned < T , R , C > where DefaultAllocator : Allocator < R , C > ; # [doc = " Clones this data storage to one that does not contain any reference."] fn clone_owned (& self) -> Owned < T , R , C > where DefaultAllocator : Allocator < R , C > ; # [doc = " Drops the storage without calling the destructors on the contained elements."] fn forget_elements (self) ; }
};
}
