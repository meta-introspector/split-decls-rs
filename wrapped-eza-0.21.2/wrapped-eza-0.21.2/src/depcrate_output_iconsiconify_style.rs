// Generated macro for iconify_style (function)
macro_rules! Depcrate_output_iconsiconify_style {
() => {
// Module: crate::output::icons
// Provides: {"iconify_style"}
// Dependencies: {}
# [doc = " Converts the style used to paint a file name into the style that should be"] # [doc = " used to paint an icon."] # [doc = ""] # [doc = " - The background colour should be preferred to the foreground colour, as"] # [doc = "   if one is set, it’s the more “obvious” colour choice."] # [doc = " - If neither is set, just use the default style."] # [doc = " - Attributes such as bold or underline should not be used to paint the"] # [doc = "   icon, as they can make it look weird."] pub fn iconify_style (style : Style) -> Style { style . background . or (style . foreground) . map (Style :: from) . unwrap_or_default () }
};
}
