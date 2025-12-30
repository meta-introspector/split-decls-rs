// Generated macro for string_wrapper (macro)
macro_rules! Depcratestring_wrapper {
() => {
// Module: crate
// Provides: {"string_wrapper"}
// Dependencies: {}
macro_rules ! string_wrapper { ($ (# [$ comment : meta]) * $ t : ident , $ towned : ident , $ validate : ident) => { string_wrapper_base ! ($ (# [$ comment]) * $ t , $ towned) ; impl StringLike for $ t { const NAME : &'static str = stringify ! ($ t) ; fn new_unchecked (s : & str) -> & Self { unsafe { std :: mem :: transmute (s) } } fn new_unchecked_owned (s : String) -> $ towned { $ towned (s) } fn is_valid (s : & str) -> Result < () , InvalidStringError > { validity ::$ validate (s . as_bytes ()) . map_err (| _ | InvalidStringError (Self :: NAME)) } } impl <'a > From <&'a $ t > for &'a DBusStr { fn from (s : &'a $ t) -> &'a DBusStr { DBusStr :: new_unchecked (&* s) } } impl <'a > TryFrom <&'a DBusStr > for &'a $ t { type Error = InvalidStringError ; fn try_from (s : &'a DBusStr) -> Result <&'a $ t , Self :: Error > { $ t :: new (&* s) } } impl AsRef < DBusStr > for $ t { fn as_ref (& self) -> & DBusStr { DBusStr :: new_unchecked (self) } } impl $ t { # [doc = " Type conversion to DBusStr."] pub fn as_dbus_str (& self) -> & DBusStr { DBusStr :: new_unchecked (self) } } impl From <$ towned > for DBusString { fn from (s : $ towned) -> DBusString { DBusStr :: new_unchecked_owned (s . into_inner ()) } } impl TryFrom < DBusString > for $ towned { type Error = InvalidStringError ; fn try_from (s : DBusString) -> Result <$ towned , Self :: Error > { $ t :: new_owned (s . into_inner ()) } } } }
};
}
