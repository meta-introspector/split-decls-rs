// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl TryFrom < Bytes > for ByteString { type Error = str :: Utf8Error ; # [inline] fn try_from (value : Bytes) -> Result < Self , Self :: Error > { let _ = str :: from_utf8 (value . as_ref ()) ? ; Ok (ByteString (value)) } }
};
}
