// Generated macro for from_bytes_cobs (function)
macro_rules! Depcrate_defrom_bytes_cobs {
() => {
// Module: crate::de
// Provides: {"from_bytes_cobs"}
// Dependencies: {}
# [doc = " Deserialize a message of type `T` from a cobs-encoded byte slice."] # [doc = ""] # [doc = " The unused portion (if any) of the byte slice is not returned."] # [doc = " The used portion of the input slice is modified during deserialization (even if an error is returned)."] # [doc = " Therefore, if this is not desired, pass a clone of the original slice."] pub fn from_bytes_cobs < 'a , T > (s : & 'a mut [u8]) -> Result < T > where T : Deserialize < 'a > , { let sz = decode_in_place (s) . map_err (| _ | Error :: DeserializeBadEncoding) ? ; from_bytes :: < T > (& s [.. sz]) }
};
}
