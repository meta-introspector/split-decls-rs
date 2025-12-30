// Generated macro for Label (function)
macro_rules! Depcrate_proxyLabel {
() => {
// Module: crate::proxy
// Provides: {"Label"}
// Dependencies: {}
# [doc = " Generic constructor for `Label`"] # [allow (clippy :: inline_always)] # [inline (always)] pub fn Label < S > (string : S) -> LabelType where S : Into < Cow < 'static , str > > , { LabelType (string . into ()) }
};
}
