// Generated macro for SameShapeStorage (type)
macro_rules! Depcrate_base_storageSameShapeStorage {
() => {
// Module: crate::base::storage
// Provides: {"SameShapeStorage"}
// Dependencies: {}
# [doc = " The data storage for the sum of two matrices with dimensions `(R1, C1)` and `(R2, C2)`."] pub type SameShapeStorage < T , R1 , C1 , R2 , C2 > = < DefaultAllocator as Allocator < SameShapeR < R1 , R2 > , SameShapeC < C1 , C2 > > > :: Buffer < T > ;
};
}
