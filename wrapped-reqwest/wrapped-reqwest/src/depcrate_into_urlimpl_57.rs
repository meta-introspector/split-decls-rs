// Generated macro for impl_57 (impl)
macro_rules! Depcrate_into_urlimpl_57 {
() => {
// Module: crate::into_url
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a > IntoUrlSealed for & 'a str { fn into_url (self) -> crate :: Result < Url > { Url :: parse (self) . map_err (crate :: error :: builder) ? . into_url () } fn as_str (& self) -> & str { self } }
};
}
