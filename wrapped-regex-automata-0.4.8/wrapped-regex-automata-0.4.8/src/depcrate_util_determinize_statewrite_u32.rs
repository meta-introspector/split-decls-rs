// Generated macro for write_u32 (function)
macro_rules! Depcrate_util_determinize_statewrite_u32 {
() => {
// Module: crate::util::determinize::state
// Provides: {"write_u32"}
// Dependencies: {}
# [doc = " Push a native-endian encoded `n` on to `dst`."] fn write_u32 (dst : & mut Vec < u8 > , n : u32) { use crate :: util :: wire :: NE ; let start = dst . len () ; dst . extend (core :: iter :: repeat (0) . take (mem :: size_of :: < u32 > ())) ; NE :: write_u32 (n , & mut dst [start ..]) ; }
};
}
