// Generated macro for u64x4 (trait)
macro_rules! Depcrate_typesu64x4 {
() => {
// Module: crate::types
// Provides: {"u64x4"}
// Dependencies: {}
pub trait u64x4 < M : Machine > : BitOps64 + Store < vec256_storage > + Vec4 < u64 > + MultiLane < [u64 ; 4] > + ArithOps + Words4 + StoreBytes + Into < vec256_storage > { }
};
}
