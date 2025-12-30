// Generated macro for impl_218 (impl)
macro_rules! Depcrate_stringsimpl_218 {
() => {
// Module: crate::strings
// Provides: {"impl_218"}
// Dependencies: {}
impl TryFrom < & str > for InlineStr { type Error = StringTooLongError ; fn try_from (s : & str) -> Result < InlineStr , StringTooLongError > { let len = s . len () ; if len <= MAX_INLINE_STR_LEN { let mut inner = [0u8 ; MAX_INLINE_STR_LEN] ; inner [.. len] . copy_from_slice (s . as_bytes ()) ; let len = len as u8 ; Ok (Self { inner , len }) } else { Err (StringTooLongError) } } }
};
}
