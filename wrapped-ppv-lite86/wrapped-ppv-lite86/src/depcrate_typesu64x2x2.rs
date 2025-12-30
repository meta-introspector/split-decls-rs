// Generated macro for u64x2x2 (trait)
macro_rules! Depcrate_typesu64x2x2 {
() => {
// Module: crate::types
// Provides: {"u64x2x2"}
// Dependencies: {}
pub trait u64x2x2 < M : Machine > : BitOps64 + Store < vec256_storage > + Vec2 < M :: u64x2 > + MultiLane < [M :: u64x2 ; 2] > + ArithOps + StoreBytes + Into < vec256_storage > { }
};
}
