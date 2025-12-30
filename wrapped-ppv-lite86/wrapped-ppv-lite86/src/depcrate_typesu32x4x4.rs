// Generated macro for u32x4x4 (trait)
macro_rules! Depcrate_typesu32x4x4 {
() => {
// Module: crate::types
// Provides: {"u32x4x4"}
// Dependencies: {}
pub trait u32x4x4 < M : Machine > : BitOps32 + Store < vec512_storage > + Vec4 < M :: u32x4 > + Vec4Ext < M :: u32x4 > + Vector < [u32 ; 16] > + MultiLane < [M :: u32x4 ; 4] > + ArithOps + LaneWords4 + Into < vec512_storage > + StoreBytes { }
};
}
