// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl StringLike for DBusStr { const NAME : & 'static str = "DBusStr" ; fn new_unchecked (s : & str) -> & Self { unsafe { std :: mem :: transmute (s) } } fn new_unchecked_owned (s : String) -> DBusString { DBusString (s) } fn is_valid (s : & str) -> Result < () , InvalidStringError > { validity :: is_valid_string (s) . map_err (| _ | InvalidStringError (Self :: NAME)) } }
};
}
