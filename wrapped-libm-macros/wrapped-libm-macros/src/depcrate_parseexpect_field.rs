// Generated macro for expect_field (function)
macro_rules! Depcrate_parseexpect_field {
() => {
// Module: crate::parse
// Provides: {"expect_field"}
// Dependencies: {}
# [doc = " Extract a named field from a map, raising an error if it doesn't exist."] fn expect_field (v : & mut Vec < Mapping > , name : & str) -> syn :: Result < Expr > { let pos = v . iter () . position (| v | v . name == name) . ok_or_else (| | { syn :: Error :: new (Span :: call_site () , format ! ("missing expected field `{name}`") ,) }) ? ; Ok (v . remove (pos) . expr) }
};
}
