// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > Parse < 'a > { # [doc = " Return a new iterator that yields pairs of `String` instead of pairs of `Cow<str>`."] pub fn into_owned (self) -> ParseIntoOwned < 'a > { ParseIntoOwned { inner : self } } }
};
}
