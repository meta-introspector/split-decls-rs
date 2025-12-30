// Generated macro for Font (function)
macro_rules! Depcrate_proxyFont {
() => {
// Module: crate::proxy
// Provides: {"Font"}
// Dependencies: {}
# [doc = " Generic constructor for `Font`"] # [allow (clippy :: inline_always)] # [inline (always)] pub fn Font < S > (string : S) -> FontType where S : Into < Cow < 'static , str > > , { FontType (string . into ()) }
};
}
