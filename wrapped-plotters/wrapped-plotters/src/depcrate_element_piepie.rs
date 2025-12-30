// Generated macro for Pie (struct)
macro_rules! Depcrate_element_piePie {
() => {
// Module: crate::element::pie
// Provides: {"Pie"}
// Dependencies: {}
# [doc = " A Pie Graph"] pub struct Pie < 'a , Coord , Label : Display > { center : & 'a Coord , radius : & 'a f64 , sizes : & 'a [f64] , colors : & 'a [RGBColor] , labels : & 'a [Label] , total : f64 , start_radian : f64 , label_style : TextStyle < 'a > , label_offset : f64 , percentage_style : Option < TextStyle < 'a > > , donut_hole : f64 , }
};
}
