// Generated macro for impl_101 (impl)
macro_rules! Depcrate_canvasimpl_101 {
() => {
// Module: crate::canvas
// Provides: {"impl_101"}
// Dependencies: {}
impl Grid for HalfBlockGrid { fn resolution (& self) -> (f64 , f64) { (f64 :: from (self . width) , f64 :: from (self . height) * 2.0) } fn save (& self) -> Layer { let vertical_color_pairs = self . pixels . iter () . tuples () . flat_map (| (upper_row , lower_row) | zip (upper_row , lower_row)) ; let string = vertical_color_pairs . clone () . map (| (upper , lower) | match (upper , lower) { (Color :: Reset , Color :: Reset) => ' ' , (Color :: Reset , _) => symbols :: half_block :: LOWER , (_ , Color :: Reset) => symbols :: half_block :: UPPER , (& lower , & upper) => { if lower == upper { symbols :: half_block :: FULL } else { symbols :: half_block :: UPPER } } }) . collect () ; let colors = vertical_color_pairs . map (| (upper , lower) | { let (fg , bg) = match (upper , lower) { (Color :: Reset , Color :: Reset) => (Color :: Reset , Color :: Reset) , (Color :: Reset , & lower) => (lower , Color :: Reset) , (& upper , Color :: Reset) => (upper , Color :: Reset) , (& upper , & lower) => (upper , lower) , } ; (fg , bg) }) . collect () ; Layer { string , colors } } fn reset (& mut self) { self . pixels . fill (vec ! [Color :: Reset ; self . width as usize]) ; } fn paint (& mut self , x : usize , y : usize , color : Color) { self . pixels [y] [x] = color ; } }
};
}
