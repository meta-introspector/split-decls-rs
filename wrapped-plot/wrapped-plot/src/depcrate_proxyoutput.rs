// Generated macro for Output (function)
macro_rules! Depcrate_proxyOutput {
() => {
// Module: crate::proxy
// Provides: {"Output"}
// Dependencies: {}
# [doc = " Generic constructor for `Output`"] # [allow (clippy :: inline_always)] # [inline (always)] pub fn Output < P > (path : P) -> OutputType where P : Into < Cow < 'static , Path > > , { OutputType (path . into ()) }
};
}
