// Generated macro for impl_940 (impl)
macro_rules! Depcrate_util_idimpl_940 {
() => {
// Module: crate::util::id
// Provides: {"impl_940"}
// Dependencies: {}
impl Id { pub (crate) const HELP : & 'static str = "help" ; pub (crate) const VERSION : & 'static str = "version" ; pub (crate) const EXTERNAL : & 'static str = "" ; pub (crate) fn from_static_ref (name : & 'static str) -> Self { Self (Str :: from_static_ref (name)) } # [doc = " Get the raw string of the `Id`"] pub fn as_str (& self) -> & str { self . 0 . as_str () } pub (crate) fn as_internal_str (& self) -> & Str { & self . 0 } }
};
}
