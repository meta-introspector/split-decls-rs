// Generated macro for Owned (type)
macro_rules! Depcrate_base_storageOwned {
() => {
// Module: crate::base::storage
// Provides: {"Owned"}
// Dependencies: {}
# [doc = " The owned data storage that can be allocated from `S`."] pub type Owned < T , R , C = U1 > = < DefaultAllocator as Allocator < R , C > > :: Buffer < T > ;
};
}
