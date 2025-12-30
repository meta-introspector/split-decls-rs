// Generated macro for Title (function)
macro_rules! Depcrate_proxyTitle {
() => {
// Module: crate::proxy
// Provides: {"Title"}
// Dependencies: {}
# [doc = " Generic constructor for `Title`"] # [allow (clippy :: inline_always)] # [inline (always)] pub fn Title < S > (string : S) -> TitleType where S : Into < Cow < 'static , str > > , { TitleType (string . into ()) }
};
}
