// Generated macro for parse_bounded_type (function)
macro_rules! Depcrate_momoparse_bounded_type {
() => {
// Module: crate::momo
// Provides: {"parse_bounded_type"}
// Dependencies: {}
fn parse_bounded_type (ty : & Type) -> Option < Ident > { match & ty { Type :: Path (TypePath { qself : None , path }) if path . segments . len () == 1 => Some (path . segments [0] . ident . clone ()) , _ => None , } }
};
}
