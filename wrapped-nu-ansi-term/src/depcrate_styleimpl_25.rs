// Generated macro for impl_25 (impl)
macro_rules! Depcrate_styleimpl_25 {
() => {
// Module: crate::style
// Provides: {"impl_25"}
// Dependencies: {}
impl Default for Style { # [doc = " Returns a style with *no* properties set. Formatting text using this"] # [doc = " style returns the exact same text."] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::Style;"] # [doc = " assert_eq!(None,  Style::default().foreground);"] # [doc = " assert_eq!(None,  Style::default().background);"] # [doc = " assert_eq!(false, Style::default().is_bold);"] # [doc = " assert_eq!(\"txt\", Style::default().paint(\"txt\").to_string());"] # [doc = " ```"] fn default () -> Style { Style { foreground : None , background : None , is_bold : false , is_dimmed : false , is_italic : false , is_underline : false , is_blink : false , is_reverse : false , is_hidden : false , is_strikethrough : false , prefix_with_reset : false , } } }
};
}
