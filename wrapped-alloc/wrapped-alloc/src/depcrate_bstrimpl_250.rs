// Generated macro for impl_250 (impl)
macro_rules! Depcrate_bstrimpl_250 {
() => {
// Module: crate::bstr
// Provides: {"impl_250"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > TryFrom < & 'a ByteStr > for String { type Error = core :: str :: Utf8Error ; # [inline] fn try_from (s : & 'a ByteStr) -> Result < Self , Self :: Error > { Ok (core :: str :: from_utf8 (& s . 0) ? . into ()) } }
};
}
