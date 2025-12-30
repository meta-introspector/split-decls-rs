// Generated macro for u32x4x2 (trait)
macro_rules! Depcrate_typesu32x4x2 {
() => {
// Module: crate::types
// Provides: {"u32x4x2"}
// Dependencies: {}
pub trait u32x4x2 < M : Machine > : BitOps32 + Store < vec256_storage > + Vec2 < M :: u32x4 > + MultiLane < [M :: u32x4 ; 2] > + ArithOps + Into < vec256_storage > + StoreBytes { }
};
}
