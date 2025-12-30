// Generated macro for to_yansi_style (function)
macro_rules! Depcrateto_yansi_style {
() => {
// Module: crate
// Provides: {"to_yansi_style"}
// Dependencies: {}
# [doc = " Adapt generic styling to [`yansi`]"] pub fn to_yansi_style (style : anstyle :: Style) -> yansi :: Style { let fg = style . get_fg_color () . map (to_yansi_color) . unwrap_or (yansi :: Color :: Primary) ; let bg = style . get_bg_color () . map (to_yansi_color) . unwrap_or (yansi :: Color :: Primary) ; let effects = style . get_effects () ; let mut style = yansi :: Style :: new () . fg (fg) . bg (bg) ; if effects . contains (anstyle :: Effects :: BOLD) { style = style . bold () ; } if effects . contains (anstyle :: Effects :: DIMMED) { style = style . dim () ; } if effects . contains (anstyle :: Effects :: ITALIC) { style = style . italic () ; } if effects . contains (anstyle :: Effects :: UNDERLINE) { style = style . underline () ; } if effects . contains (anstyle :: Effects :: BLINK) { style = style . blink () ; } if effects . contains (anstyle :: Effects :: INVERT) { style = style . invert () ; } if effects . contains (anstyle :: Effects :: HIDDEN) { style = style . conceal () ; } if effects . contains (anstyle :: Effects :: STRIKETHROUGH) { style = style . strike () ; } style }
};
}
