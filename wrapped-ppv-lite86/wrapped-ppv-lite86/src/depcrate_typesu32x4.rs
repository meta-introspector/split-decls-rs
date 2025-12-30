// Generated macro for u32x4 (trait)
macro_rules! Depcrate_typesu32x4 {
() => {
// Module: crate::types
// Provides: {"u32x4"}
// Dependencies: {}
pub trait u32x4 < M : Machine > : BitOps32 + Store < vec128_storage > + ArithOps + Vec4 < u32 > + Words4 + LaneWords4 + StoreBytes + MultiLane < [u32 ; 4] > + Into < vec128_storage > { }
};
}
