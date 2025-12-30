// Generated macro for theme_setters (macro)
macro_rules! Depcrate_configtheme_setters {
() => {
// Module: crate::config
// Provides: {"theme_setters"}
// Dependencies: {}
macro_rules ! theme_setters { ($ (# [$ meta : meta] $ name : ident) ,* $ (,) ?) => { $ (# [$ meta] pub fn $ name (mut self , style : Style) -> Self { self .$ name = style ; self }) * } ; }
};
}
