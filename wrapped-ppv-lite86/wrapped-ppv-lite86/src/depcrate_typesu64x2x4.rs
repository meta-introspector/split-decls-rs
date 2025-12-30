// Generated macro for u64x2x4 (trait)
macro_rules! Depcrate_typesu64x2x4 {
() => {
// Module: crate::types
// Provides: {"u64x2x4"}
// Dependencies: {}
pub trait u64x2x4 < M : Machine > : BitOps64 + Store < vec512_storage > + Vec4 < M :: u64x2 > + MultiLane < [M :: u64x2 ; 4] > + ArithOps + Into < vec512_storage > { }
};
}
