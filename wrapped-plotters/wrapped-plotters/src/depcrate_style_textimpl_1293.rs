// Generated macro for impl_1293 (impl)
macro_rules! Depcrate_style_textimpl_1293 {
() => {
// Module: crate::style::text
// Provides: {"impl_1293"}
// Dependencies: {}
# [doc = " Make sure that we are able to automatically copy the `TextStyle`"] impl < 'a , 'b : 'a > From < & 'b TextStyle < 'a > > for TextStyle < 'a > { fn from (this : & 'b TextStyle < 'a >) -> Self { this . clone () } }
};
}
