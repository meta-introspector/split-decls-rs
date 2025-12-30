// Generated macro for from_i64 (macro)
macro_rules! Depcrate_types_valuefrom_i64 {
() => {
// Module: crate::types::value
// Provides: {"from_i64"}
// Dependencies: {}
macro_rules ! from_i64 (($ t : ty) => (impl From <$ t > for Value { # [inline] fn from (i : $ t) -> Value { Value :: Integer (i64 :: from (i)) } })) ;
};
}
