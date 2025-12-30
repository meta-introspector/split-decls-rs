// Generated macro for CStride (type)
macro_rules! Depcrate_base_storageCStride {
() => {
// Module: crate::base::storage
// Provides: {"CStride"}
// Dependencies: {}
# [doc = " The column-stride of the owned data storage for a buffer of dimension `(R, C)`."] pub type CStride < T , R , C = U1 > = < < DefaultAllocator as Allocator < R , C > > :: Buffer < T > as RawStorage < T , R , C > > :: CStride ;
};
}
