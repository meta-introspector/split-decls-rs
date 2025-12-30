// Generated macro for to_termcolor_spec (function)
macro_rules! Depcrateto_termcolor_spec {
() => {
// Module: crate
// Provides: {"to_termcolor_spec"}
// Dependencies: {}
# [doc = " Adapt generic styling to [`termcolor`]"] pub fn to_termcolor_spec (style : anstyle :: Style) -> termcolor :: ColorSpec { let fg = style . get_fg_color () . map (to_termcolor_color) ; let bg = style . get_bg_color () . map (to_termcolor_color) ; let effects = style . get_effects () ; let mut style = termcolor :: ColorSpec :: new () ; style . set_fg (fg) ; style . set_bg (bg) ; style . set_bold (effects . contains (anstyle :: Effects :: BOLD)) ; style . set_dimmed (effects . contains (anstyle :: Effects :: DIMMED)) ; style . set_italic (effects . contains (anstyle :: Effects :: ITALIC)) ; style . set_underline (effects . contains (anstyle :: Effects :: UNDERLINE)) ; style }
};
}
