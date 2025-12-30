// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl TryFrom < bytes :: BytesMut > for ByteString { type Error = str :: Utf8Error ; # [inline] fn try_from (value : bytes :: BytesMut) -> Result < Self , Self :: Error > { let _ = str :: from_utf8 (& value) ? ; Ok (ByteString (value . freeze ())) } }
};
}
