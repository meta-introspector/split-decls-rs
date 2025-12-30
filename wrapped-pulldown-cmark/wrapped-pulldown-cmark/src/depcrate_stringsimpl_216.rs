// Generated macro for impl_216 (impl)
macro_rules! Depcrate_stringsimpl_216 {
() => {
// Module: crate::strings
// Provides: {"impl_216"}
// Dependencies: {}
impl From < char > for InlineStr { fn from (c : char) -> Self { let mut inner = [0u8 ; MAX_INLINE_STR_LEN] ; c . encode_utf8 (& mut inner) ; let len = c . len_utf8 () as u8 ; Self { inner , len } } }
};
}
