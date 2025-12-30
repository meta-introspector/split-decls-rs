// Generated macro for to_owo_style (function)
macro_rules! Depcrateto_owo_style {
() => {
// Module: crate
// Provides: {"to_owo_style"}
// Dependencies: {}
# [doc = " Adapt generic styling to [`owo_colors`]"] pub fn to_owo_style (style : anstyle :: Style) -> owo_colors :: Style { let fg = style . get_fg_color () . map (to_owo_colors) ; let bg = style . get_bg_color () . map (to_owo_colors) ; let effects = style . get_effects () ; let mut style = owo_colors :: Style :: new () ; if let Some (fg) = fg { style = style . color (fg) ; } if let Some (bg) = bg { style = style . on_color (bg) ; } if effects . contains (anstyle :: Effects :: BOLD) { style = style . bold () ; } if effects . contains (anstyle :: Effects :: DIMMED) { style = style . dimmed () ; } if effects . contains (anstyle :: Effects :: ITALIC) { style = style . italic () ; } if effects . contains (anstyle :: Effects :: UNDERLINE) { style = style . underline () ; } if effects . contains (anstyle :: Effects :: BLINK) { style = style . blink () ; } if effects . contains (anstyle :: Effects :: INVERT) { style = style . reversed () ; } if effects . contains (anstyle :: Effects :: HIDDEN) { style = style . hidden () ; } if effects . contains (anstyle :: Effects :: STRIKETHROUGH) { style = style . strikethrough () ; } style }
};
}
