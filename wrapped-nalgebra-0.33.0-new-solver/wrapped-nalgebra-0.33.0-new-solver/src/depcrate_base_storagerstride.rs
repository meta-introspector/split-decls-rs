// Generated macro for RStride (type)
macro_rules! Depcrate_base_storageRStride {
() => {
// Module: crate::base::storage
// Provides: {"RStride"}
// Dependencies: {}
# [doc = " The row-stride of the owned data storage for a buffer of dimension `(R, C)`."] pub type RStride < T , R , C = U1 > = < < DefaultAllocator as Allocator < R , C > > :: Buffer < T > as RawStorage < T , R , C > > :: RStride ;
};
}
