// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl FromTermwiz < CellAttributes > for Style { fn from_termwiz (value : CellAttributes) -> Self { let mut style = Self :: new () . add_modifier (value . intensity () . into_ratatui ()) . add_modifier (value . underline () . into_ratatui ()) . add_modifier (value . blink () . into_ratatui ()) ; if value . italic () { style . add_modifier |= Modifier :: ITALIC ; } if value . reverse () { style . add_modifier |= Modifier :: REVERSED ; } if value . strikethrough () { style . add_modifier |= Modifier :: CROSSED_OUT ; } if value . invisible () { style . add_modifier |= Modifier :: HIDDEN ; } style . fg = Some (value . foreground () . into_ratatui ()) ; style . bg = Some (value . background () . into_ratatui ()) ; # [cfg (feature = "underline-color")] { style . underline_color = Some (value . underline_color () . into_ratatui ()) ; } style } }
};
}
