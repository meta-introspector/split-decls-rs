// Generated macro for StorageMut (trait)
macro_rules! Depcrate_base_storageStorageMut {
() => {
// Module: crate::base::storage
// Provides: {"StorageMut"}
// Dependencies: {}
# [doc = " Trait shared by all mutable matrix data storage that don’t contain any uninitialized elements."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See safety note for `Storage`, `RawStorageMut`."] pub unsafe trait StorageMut < T : Scalar , R : Dim , C : Dim = U1 > : Storage < T , R , C > + RawStorageMut < T , R , C > { }
};
}
