// Generated macro for impl_107 (impl)
macro_rules! Depcrate_jsontimpl_107 {
() => {
// Module: crate::jsont
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > Data < 'a > { fn from_bytes (bytes : & [u8]) -> Data < '_ > { match std :: str :: from_utf8 (bytes) { Ok (text) => Data :: Text { text : Cow :: Borrowed (text) } , Err (_) => Data :: Bytes { bytes } , } } # [cfg (unix)] fn from_path (path : & Path) -> Data < '_ > { use std :: os :: unix :: ffi :: OsStrExt ; match path . to_str () { Some (text) => Data :: Text { text : Cow :: Borrowed (text) } , None => Data :: Bytes { bytes : path . as_os_str () . as_bytes () } , } } # [cfg (not (unix))] fn from_path (path : & Path) -> Data < '_ > { Data :: Text { text : path . to_string_lossy () } } }
};
}
