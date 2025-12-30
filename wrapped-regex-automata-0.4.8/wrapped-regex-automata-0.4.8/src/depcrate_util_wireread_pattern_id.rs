// Generated macro for read_pattern_id (function)
macro_rules! Depcrate_util_wireread_pattern_id {
() => {
// Module: crate::util::wire
// Provides: {"read_pattern_id"}
// Dependencies: {}
# [doc = " Reads a pattern ID from the given slice. If the slice has insufficient"] # [doc = " length, then this panics. If the deserialized integer exceeds the pattern"] # [doc = " ID limit for the current target, then this returns an error."] # [doc = ""] # [doc = " Upon success, this also returns the number of bytes read."] pub (crate) fn read_pattern_id (slice : & [u8] , what : & 'static str ,) -> Result < (PatternID , usize) , DeserializeError > { let bytes : [u8 ; PatternID :: SIZE] = slice [.. PatternID :: SIZE] . try_into () . unwrap () ; let pid = PatternID :: from_ne_bytes (bytes) . map_err (| err | DeserializeError :: pattern_id_error (err , what)) ? ; Ok ((pid , PatternID :: SIZE)) }
};
}
