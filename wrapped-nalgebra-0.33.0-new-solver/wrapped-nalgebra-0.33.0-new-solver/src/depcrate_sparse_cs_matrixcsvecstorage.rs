// Generated macro for CsVecStorage (struct)
macro_rules! Depcrate_sparse_cs_matrixCsVecStorage {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"CsVecStorage"}
// Dependencies: {}
# [doc = " A storage of column-compressed sparse matrix based on a Vec."] # [derive (Clone , Debug , PartialEq)] pub struct CsVecStorage < T : Scalar , R : Dim , C : Dim > where DefaultAllocator : Allocator < C > , { pub (crate) shape : (R , C) , pub (crate) p : OVector < usize , C > , pub (crate) i : Vec < usize > , pub (crate) vals : Vec < T > , }
};
}
