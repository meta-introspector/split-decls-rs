// Generated macro for ReshapableStorage (trait)
macro_rules! Depcrate_base_storageReshapableStorage {
() => {
// Module: crate::base::storage
// Provides: {"ReshapableStorage"}
// Dependencies: {}
# [doc = " A matrix storage that can be reshaped in-place."] pub trait ReshapableStorage < T , R1 , C1 , R2 , C2 > : RawStorage < T , R1 , C1 > where T : Scalar , R1 : Dim , C1 : Dim , R2 : Dim , C2 : Dim , { # [doc = " The reshaped storage type."] type Output : RawStorage < T , R2 , C2 > ; # [doc = " Reshapes the storage into the output storage type."] fn reshape_generic (self , nrows : R2 , ncols : C2) -> Self :: Output ; }
};
}
