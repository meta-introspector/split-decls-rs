// Generated macro for to_ansi_term (function)
macro_rules! Depcrateto_ansi_term {
() => {
// Module: crate
// Provides: {"to_ansi_term"}
// Dependencies: {}
# [doc = " Adapt generic styling to [`ansi_term`]"] pub fn to_ansi_term (astyle : anstyle :: Style) -> ansi_term :: Style { let mut style = ansi_term :: Style :: new () ; if let Some ((fg , fg_bold)) = astyle . get_fg_color () . map (to_ansi_color) { style = style . fg (fg) ; if fg_bold { style = style . bold () ; } } if let Some ((bg , _)) = astyle . get_bg_color () . map (to_ansi_color) { style = style . on (bg) ; } let effects = astyle . get_effects () ; if effects . contains (anstyle :: Effects :: BOLD) { style = style . bold () ; } if effects . contains (anstyle :: Effects :: DIMMED) { style = style . dimmed () ; } if effects . contains (anstyle :: Effects :: ITALIC) { style = style . italic () ; } if effects . contains (anstyle :: Effects :: UNDERLINE) { style = style . underline () ; } if effects . contains (anstyle :: Effects :: BLINK) { style = style . blink () ; } if effects . contains (anstyle :: Effects :: INVERT) { style = style . reverse () ; } if effects . contains (anstyle :: Effects :: HIDDEN) { style = style . hidden () ; } if effects . contains (anstyle :: Effects :: STRIKETHROUGH) { style = style . strikethrough () ; } style }
};
}
