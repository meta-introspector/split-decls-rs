// Generated macro for impl_182 (impl)
macro_rules! Depcrateimpl_182 {
() => {
// Module: crate
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a > From < & 'a JsString > for String { fn from (s : & 'a JsString) -> Self { s . obj . as_string () . unwrap_throw () } }
};
}
