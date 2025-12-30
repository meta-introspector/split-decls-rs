// Generated macro for impl_235 (impl)
macro_rules! Depcrate_bstrimpl_235 {
() => {
// Module: crate::bstr
// Provides: {"impl_235"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl TryFrom < ByteString > for String { type Error = crate :: string :: FromUtf8Error ; # [inline] fn try_from (s : ByteString) -> Result < Self , Self :: Error > { String :: from_utf8 (s . 0) } }
};
}
