// Generated macro for OwnedUninit (type)
macro_rules! Depcrate_base_storageOwnedUninit {
() => {
// Module: crate::base::storage
// Provides: {"OwnedUninit"}
// Dependencies: {}
# [doc = " The owned data storage that can be allocated from `S`."] pub type OwnedUninit < T , R , C = U1 > = < DefaultAllocator as Allocator < R , C > > :: BufferUninit < T > ;
};
}
