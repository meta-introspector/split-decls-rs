// Generated macro for ttf (module)
macro_rules! Depcrate_style_fontttf {
() => {
// Module: crate::style::font
// Provides: {"ttf"}
// Dependencies: {}
# [doc = " The implementation of an actual font implementation"] # [doc = ""] # [doc = " This exists since for the image rendering task, we want to use"] # [doc = " the system font. But in wasm application, we want the browser"] # [doc = " to handle all the font issue."] # [doc = ""] # [doc = " Thus we need different mechanism for the font implementation"] # [cfg (all (not (all (target_arch = "wasm32" , not (target_os = "wasi"))) , feature = "ttf"))] mod ttf ;
};
}
