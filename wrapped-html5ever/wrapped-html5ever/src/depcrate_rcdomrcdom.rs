// Generated macro for RcDom (struct)
macro_rules! Depcrate_rcdomRcDom {
() => {
// Module: crate::rcdom
// Provides: {"RcDom"}
// Dependencies: {}
# [doc = " The DOM itself; the result of parsing."] pub struct RcDom { # [doc = " The `Document` itself."] pub document : Handle , # [doc = " Errors that occurred during parsing."] pub errors : Vec < Cow < 'static , str > > , # [doc = " The document's quirks mode."] pub quirks_mode : QuirksMode , }
};
}
