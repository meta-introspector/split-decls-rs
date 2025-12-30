// Generated macro for impl_525 (impl)
macro_rules! Depcrate_parserimpl_525 {
() => {
// Module: crate::parser
// Provides: {"impl_525"}
// Dependencies: {}
impl NameValueParser { pub fn value_as_lit (& self) -> & MetaItemLit { & self . value } pub fn value_as_str (& self) -> Option < Symbol > { self . value_as_lit () . kind . str () } }
};
}
