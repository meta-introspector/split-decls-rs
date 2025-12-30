// Generated macro for impl_129 (impl)
macro_rules! Depcrate_schemaimpl_129 {
() => {
// Module: crate::schema
// Provides: {"impl_129"}
// Dependencies: {}
impl Definition { # [doc = " Array length isn't present in payload, it's determined by type of data"] # [doc = " serialized."] pub const ARRAY_LENGTH_WIDTH : u8 = 0 ; # [doc = " Convenience constant representing the length width of a standard borsh"] # [doc = " sequence."] # [doc = ""] # [doc = " Can be used for `Definition::Sequence::length_width`."] pub const DEFAULT_LENGTH_WIDTH : u8 = 4 ; # [doc = " Convenience constant representing the length range of a standard borsh"] # [doc = " sequence."] # [doc = ""] # [doc = " It equals `0..=u32::MAX`.  Can be used with"] # [doc = " `Definition::Sequence::length_range`."] pub const DEFAULT_LENGTH_RANGE : core :: ops :: RangeInclusive < u64 > = 0 ..= (u32 :: MAX as u64) ; }
};
}
