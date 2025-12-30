// Generated macro for convert_primitive_bytes (macro)
macro_rules! Depcrate_convertconvert_primitive_bytes {
() => {
// Module: crate::convert
// Provides: {"convert_primitive_bytes"}
// Dependencies: {}
macro_rules ! convert_primitive_bytes { ($ a : ty , $ b : ty) => { impl Convert <$ b > for $ a { # [inline (always)] fn convert (self) -> $ b { self . to_ne_bytes () } } impl Convert <$ a > for $ b { # [inline (always)] fn convert (self) -> $ a { <$ a >:: from_ne_bytes (self) } } } ; }
};
}
