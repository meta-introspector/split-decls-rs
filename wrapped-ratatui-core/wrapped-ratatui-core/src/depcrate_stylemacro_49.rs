// Generated macro for macro_49 (macro)
macro_rules! Depcrate_stylemacro_49 {
() => {
// Module: crate::style
// Provides: {"macro_49"}
// Dependencies: {}
bitflags ! { # [doc = " Modifier changes the way a piece of text is displayed."] # [doc = ""] # [doc = " They are bitflags so they can easily be composed."] # [doc = ""] # [doc = " `From<Modifier> for Style` is implemented so you can use `Modifier` anywhere that accepts"] # [doc = " `Into<Style>`."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::Modifier;"] # [doc = ""] # [doc = " let m = Modifier::BOLD | Modifier::ITALIC;"] # [doc = " ```"] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Default , Clone , Copy , Eq , PartialEq , Hash)] pub struct Modifier : u16 { const BOLD = 0b0000_0000_0001 ; const DIM = 0b0000_0000_0010 ; const ITALIC = 0b0000_0000_0100 ; const UNDERLINED = 0b0000_0000_1000 ; const SLOW_BLINK = 0b0000_0001_0000 ; const RAPID_BLINK = 0b0000_0010_0000 ; const REVERSED = 0b0000_0100_0000 ; const HIDDEN = 0b0000_1000_0000 ; const CROSSED_OUT = 0b0001_0000_0000 ; } }
};
}
