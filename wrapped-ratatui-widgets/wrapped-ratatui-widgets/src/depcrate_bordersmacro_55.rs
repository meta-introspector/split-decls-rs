// Generated macro for macro_55 (macro)
macro_rules! Depcrate_bordersmacro_55 {
() => {
// Module: crate::borders
// Provides: {"macro_55"}
// Dependencies: {}
bitflags ! { # [doc = " Bitflags that can be composed to set the visible borders essentially on the block widget."] # [derive (Default , Clone , Copy , Eq , PartialEq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Borders : u8 { # [doc = " Show the top border"] const TOP = 0b0001 ; # [doc = " Show the right border"] const RIGHT = 0b0010 ; # [doc = " Show the bottom border"] const BOTTOM = 0b0100 ; # [doc = " Show the left border"] const LEFT = 0b1000 ; # [doc = " Show all borders"] const ALL = Self :: TOP . bits () | Self :: RIGHT . bits () | Self :: BOTTOM . bits () | Self :: LEFT . bits () ; } }
};
}
