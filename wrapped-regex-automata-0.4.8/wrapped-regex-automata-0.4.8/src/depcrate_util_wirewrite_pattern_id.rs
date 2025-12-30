// Generated macro for write_pattern_id (function)
macro_rules! Depcrate_util_wirewrite_pattern_id {
() => {
// Module: crate::util::wire
// Provides: {"write_pattern_id"}
// Dependencies: {}
# [doc = " Write the given pattern ID to the beginning of the given slice of bytes"] # [doc = " using the specified endianness. The given slice must have length at least"] # [doc = " `PatternID::SIZE`, or else this panics. Upon success, the total number of"] # [doc = " bytes written is returned."] pub (crate) fn write_pattern_id < E : Endian > (pid : PatternID , dst : & mut [u8] ,) -> usize { E :: write_u32 (pid . as_u32 () , dst) ; PatternID :: SIZE }
};
}
