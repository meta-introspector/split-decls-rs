// Generated macro for convert (macro)
macro_rules! Depcrate_convertconvert {
() => {
// Module: crate::convert
// Provides: {"convert"}
// Dependencies: {}
macro_rules ! convert { ($ a : ty , $ b : ty) => { impl Convert <$ b > for $ a { # [inline (always)] fn convert (self) -> $ b { zerocopy :: transmute ! (self) } } impl Convert <$ a > for $ b { # [inline (always)] fn convert (self) -> $ a { zerocopy :: transmute ! (self) } } } ; }
};
}
