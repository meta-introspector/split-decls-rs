// Generated macro for impl_420 (impl)
macro_rules! Depcrate_de_implsimpl_420 {
() => {
// Module: crate::de::impls
// Provides: {"impl_420"}
// Dependencies: {}
impl < Context > Decode < Context > for char { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let mut array = [0u8 ; 4] ; decoder . reader () . read (& mut array [.. 1]) ? ; let width = utf8_char_width (array [0]) ; if width == 0 { return Err (DecodeError :: InvalidCharEncoding (array)) ; } decoder . claim_bytes_read (width) ? ; if width == 1 { return Ok (array [0] as char) ; } decoder . reader () . read (& mut array [1 .. width]) ? ; let res = core :: str :: from_utf8 (& array [.. width]) . ok () . and_then (| s | s . chars () . next ()) . ok_or (DecodeError :: InvalidCharEncoding (array)) ? ; Ok (res) } }
};
}
