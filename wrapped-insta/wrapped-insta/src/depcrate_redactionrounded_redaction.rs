// Generated macro for rounded_redaction (function)
macro_rules! Depcrate_redactionrounded_redaction {
() => {
// Module: crate::redaction
// Provides: {"rounded_redaction"}
// Dependencies: {}
# [doc = " Creates a redaction that rounds floating point numbers to a given"] # [doc = " number of decimal places."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use insta::{Settings, rounded_redaction};"] # [doc = " # let mut settings = Settings::new();"] # [doc = " settings.add_redaction(\".sum\", rounded_redaction(2));"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "redactions")))] pub fn rounded_redaction (decimals : usize) -> Redaction { dynamic_redaction (move | value : Content , _path : ContentPath | -> Content { let f = match value . resolve_inner () { Content :: F32 (f) => * f as f64 , Content :: F64 (f) => * f , _ => return value , } ; let x = 10f64 . powf (decimals as f64) ; Content :: F64 ((f * x) . round () / x) }) }
};
}
