// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl FromCrossterm < ContentStyle > for Style { fn from_crossterm (value : ContentStyle) -> Self { let mut sub_modifier = Modifier :: empty () ; if value . attributes . has (CrosstermAttribute :: NoBold) { sub_modifier |= Modifier :: BOLD ; } if value . attributes . has (CrosstermAttribute :: NoItalic) { sub_modifier |= Modifier :: ITALIC ; } if value . attributes . has (CrosstermAttribute :: NotCrossedOut) { sub_modifier |= Modifier :: CROSSED_OUT ; } if value . attributes . has (CrosstermAttribute :: NoUnderline) { sub_modifier |= Modifier :: UNDERLINED ; } if value . attributes . has (CrosstermAttribute :: NoHidden) { sub_modifier |= Modifier :: HIDDEN ; } if value . attributes . has (CrosstermAttribute :: NoBlink) { sub_modifier |= Modifier :: RAPID_BLINK | Modifier :: SLOW_BLINK ; } if value . attributes . has (CrosstermAttribute :: NoReverse) { sub_modifier |= Modifier :: REVERSED ; } Self { fg : value . foreground_color . map (FromCrossterm :: from_crossterm) , bg : value . background_color . map (FromCrossterm :: from_crossterm) , # [cfg (feature = "underline-color")] underline_color : value . underline_color . map (FromCrossterm :: from_crossterm) , add_modifier : Modifier :: from_crossterm (value . attributes) , sub_modifier , } } }
};
}
