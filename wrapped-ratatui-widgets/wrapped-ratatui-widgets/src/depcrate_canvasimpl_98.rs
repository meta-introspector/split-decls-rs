// Generated macro for impl_98 (impl)
macro_rules! Depcrate_canvasimpl_98 {
() => {
// Module: crate::canvas
// Provides: {"impl_98"}
// Dependencies: {}
impl Grid for CharGrid { fn resolution (& self) -> (f64 , f64) { (f64 :: from (self . width) , f64 :: from (self . height)) } fn save (& self) -> Layer { Layer { string : self . cells . iter () . collect () , colors : self . colors . iter () . map (| c | (* c , Color :: Reset)) . collect () , } } fn reset (& mut self) { self . cells . fill (' ') ; self . colors . fill (Color :: Reset) ; } fn paint (& mut self , x : usize , y : usize , color : Color) { let index = y . saturating_mul (self . width as usize) . saturating_add (x) ; if let Some (c) = self . cells . get_mut (index) { * c = self . cell_char ; } if let Some (c) = self . colors . get_mut (index) { * c = color ; } } }
};
}
