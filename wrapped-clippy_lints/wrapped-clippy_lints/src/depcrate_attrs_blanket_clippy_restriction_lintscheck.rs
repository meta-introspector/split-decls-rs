// Generated macro for check (function)
macro_rules! Depcrate_attrs_blanket_clippy_restriction_lintscheck {
() => {
// Module: crate::attrs::blanket_clippy_restriction_lints
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , name : Symbol , items : & [MetaItemInner]) { for lint in items { if name != sym :: allow && extract_clippy_lint (lint) == Some (sym :: restriction) { span_lint_and_help (cx , BLANKET_CLIPPY_RESTRICTION_LINTS , lint . span () , "`clippy::restriction` is not meant to be enabled as a group" , None , "enable the restriction lints you need individually" ,) ; } } }
};
}
