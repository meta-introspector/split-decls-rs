// Generated macro for VecStorage (struct)
macro_rules! Depcrate_base_vec_storageVecStorage {
() => {
// Module: crate::base::vec_storage
// Provides: {"VecStorage"}
// Dependencies: {}
# [doc = " A Vec-based matrix data storage. It may be dynamically-sized."] # [repr (C)] # [derive (Eq , Debug , Clone , PartialEq)] pub struct VecStorage < T , R : Dim , C : Dim > { data : Vec < T > , nrows : R , ncols : C , }
};
}
