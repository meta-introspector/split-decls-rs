// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for ByteString { type Error = str :: Utf8Error ; # [inline] fn try_from (value : Vec < u8 >) -> Result < Self , Self :: Error > { let buf = String :: from_utf8 (value) . map_err (| err | err . utf8_error ()) ? ; Ok (ByteString (Bytes :: from (buf))) } }
};
}
