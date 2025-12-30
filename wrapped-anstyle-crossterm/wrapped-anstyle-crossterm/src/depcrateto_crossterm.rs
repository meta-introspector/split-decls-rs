// Generated macro for to_crossterm (function)
macro_rules! Depcrateto_crossterm {
() => {
// Module: crate
// Provides: {"to_crossterm"}
// Dependencies: {}
# [doc = " Adapt generic styling to [`crossterm`]"] pub fn to_crossterm (astyle : anstyle :: Style) -> crossterm :: style :: ContentStyle { let foreground_color = astyle . get_fg_color () . map (to_ansi_color) ; let background_color = astyle . get_bg_color () . map (to_ansi_color) ; let underline_color = astyle . get_underline_color () . map (to_ansi_color) ; let mut attributes = crossterm :: style :: Attributes :: default () ; let effects = astyle . get_effects () ; if effects . contains (anstyle :: Effects :: BOLD) { attributes . set (crossterm :: style :: Attribute :: Bold) ; } if effects . contains (anstyle :: Effects :: DIMMED) { attributes . set (crossterm :: style :: Attribute :: Dim) ; } if effects . contains (anstyle :: Effects :: ITALIC) { attributes . set (crossterm :: style :: Attribute :: Italic) ; } if effects . contains (anstyle :: Effects :: UNDERLINE) { attributes . set (crossterm :: style :: Attribute :: Underlined) ; } if effects . contains (anstyle :: Effects :: BLINK) { attributes . set (crossterm :: style :: Attribute :: SlowBlink) ; } if effects . contains (anstyle :: Effects :: INVERT) { attributes . set (crossterm :: style :: Attribute :: Reverse) ; } if effects . contains (anstyle :: Effects :: HIDDEN) { attributes . set (crossterm :: style :: Attribute :: Hidden) ; } if effects . contains (anstyle :: Effects :: STRIKETHROUGH) { attributes . set (crossterm :: style :: Attribute :: OverLined) ; } crossterm :: style :: ContentStyle { foreground_color , background_color , underline_color , attributes , } }
};
}
