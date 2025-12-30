// Generated macro for impl_78 (impl)
macro_rules! Depcrate_color_contextimpl_78 {
() => {
// Module: crate::color_context
// Provides: {"impl_78"}
// Dependencies: {}
impl BaseColor { # [doc = " Return the index of a color, in the same ordering as the ANSI color sequences."] # [cfg (not (feature = "terminfo"))] pub fn index (& self) -> u8 { match self { Self :: Black => 0 , Self :: Red => 1 , Self :: Green => 2 , Self :: Yellow => 3 , Self :: Blue => 4 , Self :: Magenta => 5 , Self :: Cyan => 6 , Self :: White => 7 , } } # [doc = " Used to generate terminfo constants, see [`Color16::terminfo_constant()`]."] # [cfg (feature = "terminfo")] pub fn uppercase_str (& self) -> & 'static str { match self { Self :: Black => "BLACK" , Self :: Red => "RED" , Self :: Green => "GREEN" , Self :: Yellow => "YELLOW" , Self :: Blue => "BLUE" , Self :: Magenta => "MAGENTA" , Self :: Cyan => "CYAN" , Self :: White => "WHITE" , } } }
};
}
