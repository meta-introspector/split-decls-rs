// Generated macro for impl_95 (impl)
macro_rules! Depcrate_canvasimpl_95 {
() => {
// Module: crate::canvas
// Provides: {"impl_95"}
// Dependencies: {}
impl Grid for BrailleGrid { fn resolution (& self) -> (f64 , f64) { (f64 :: from (self . width) * 2.0 , f64 :: from (self . height) * 4.0) } fn save (& self) -> Layer { let string = String :: from_utf16 (& self . utf16_code_points) . unwrap () ; let colors = self . colors . iter () . map (| c | (* c , Color :: Reset)) . collect () ; Layer { string , colors } } fn reset (& mut self) { self . utf16_code_points . fill (symbols :: braille :: BLANK) ; self . colors . fill (Color :: Reset) ; } fn paint (& mut self , x : usize , y : usize , color : Color) { let index = y . saturating_div (4) . saturating_mul (self . width as usize) . saturating_add (x . saturating_div (2)) ; if let Some (c) = self . utf16_code_points . get_mut (index) { * c |= symbols :: braille :: DOTS [y % 4] [x % 2] ; } if let Some (c) = self . colors . get_mut (index) { * c = color ; } } }
};
}
