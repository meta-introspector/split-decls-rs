// Generated macro for u128x2 (trait)
macro_rules! Depcrate_typesu128x2 {
() => {
// Module: crate::types
// Provides: {"u128x2"}
// Dependencies: {}
pub trait u128x2 < M : Machine > : BitOps128 + Store < vec256_storage > + Vec2 < M :: u128x1 > + MultiLane < [M :: u128x1 ; 2] > + Swap64 + Into < vec256_storage > { }
};
}
