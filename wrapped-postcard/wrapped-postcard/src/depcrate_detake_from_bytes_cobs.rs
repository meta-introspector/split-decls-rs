// Generated macro for take_from_bytes_cobs (function)
macro_rules! Depcrate_detake_from_bytes_cobs {
() => {
// Module: crate::de
// Provides: {"take_from_bytes_cobs"}
// Dependencies: {}
# [doc = " Deserialize a message of type `T` from a cobs-encoded byte slice."] # [doc = ""] # [doc = " The unused portion (if any) of the byte slice is returned for further usage."] # [doc = " The used portion of the input slice is modified during deserialization (even if an error is returned)."] # [doc = " Therefore, if this is not desired, pass a clone of the original slice."] pub fn take_from_bytes_cobs < 'a , T > (s : & 'a mut [u8]) -> Result < (T , & 'a mut [u8]) > where T : Deserialize < 'a > , { let mut report = decode_in_place_report (s) . map_err (| _ | Error :: DeserializeBadEncoding) ? ; if s . get (report . src_used) == Some (& 0) { report . src_used += 1 ; } let (dst_used , dst_unused) = s . split_at_mut (report . dst_used) ; let (_unused , src_unused) = dst_unused . split_at_mut (report . src_used - report . dst_used) ; Ok ((from_bytes :: < T > (dst_used) ? , src_unused)) }
};
}
