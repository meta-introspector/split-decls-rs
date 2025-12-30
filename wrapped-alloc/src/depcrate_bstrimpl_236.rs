// Generated macro for impl_236 (impl)
macro_rules! Depcrate_bstrimpl_236 {
() => {
// Module: crate::bstr
// Provides: {"impl_236"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > TryFrom < & 'a ByteString > for & 'a str { type Error = crate :: str :: Utf8Error ; # [inline] fn try_from (s : & 'a ByteString) -> Result < Self , Self :: Error > { crate :: str :: from_utf8 (s . 0 . as_slice ()) } }
};
}
