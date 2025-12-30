// Generated macro for impl_219 (impl)
macro_rules! Depcrate_windows_term_colorsimpl_219 {
() => {
// Module: crate::windows_term::colors
// Provides: {"impl_219"}
// Dependencies: {}
impl Intense { fn to_bg (self) -> WORD { self . to_fg () << 4 } fn from_bg (word : WORD) -> Intense { Intense :: from_fg (word >> 4) } fn to_fg (self) -> WORD { match self { Intense :: No => 0 , Intense :: Yes => FG_INTENSITY , } } fn from_fg (word : WORD) -> Intense { if word & FG_INTENSITY > 0 { Intense :: Yes } else { Intense :: No } } }
};
}
