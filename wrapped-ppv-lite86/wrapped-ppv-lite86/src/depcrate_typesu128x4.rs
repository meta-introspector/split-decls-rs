// Generated macro for u128x4 (trait)
macro_rules! Depcrate_typesu128x4 {
() => {
// Module: crate::types
// Provides: {"u128x4"}
// Dependencies: {}
pub trait u128x4 < M : Machine > : BitOps128 + Store < vec512_storage > + Vec4 < M :: u128x1 > + MultiLane < [M :: u128x1 ; 4] > + Swap64 + Into < vec512_storage > { }
};
}
