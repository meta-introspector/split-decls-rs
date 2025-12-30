// Generated macro for impl_824 (impl)
macro_rules! Depcrate_extimpl_824 {
() => {
// Module: crate::ext
// Provides: {"impl_824"}
// Dependencies: {}
impl Protocol { # [doc = " Converts a static string to a protocol name."] pub const fn from_static (value : & 'static str) -> Self { Self { value : BytesStr :: from_static (value) , } } # [doc = " Returns a str representation of the header."] pub fn as_str (& self) -> & str { self . value . as_str () } pub (crate) fn try_from (bytes : Bytes) -> Result < Self , std :: str :: Utf8Error > { Ok (Self { value : BytesStr :: try_from (bytes) ? , }) } }
};
}
