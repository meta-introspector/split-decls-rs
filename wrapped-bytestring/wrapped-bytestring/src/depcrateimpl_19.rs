// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl TryFrom < & [u8] > for ByteString { type Error = str :: Utf8Error ; # [inline] fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { let _ = str :: from_utf8 (value) ? ; Ok (ByteString (Bytes :: copy_from_slice (value))) } }
};
}
