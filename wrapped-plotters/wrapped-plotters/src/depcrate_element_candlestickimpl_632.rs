// Generated macro for impl_632 (impl)
macro_rules! Depcrate_element_candlestickimpl_632 {
() => {
// Module: crate::element::candlestick
// Provides: {"impl_632"}
// Dependencies: {}
impl < X , Y : PartialOrd , DB : DrawingBackend > Drawable < DB > for CandleStick < X , Y > { fn draw < I : Iterator < Item = BackendCoord > > (& self , points : I , backend : & mut DB , _ : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > { let mut points : Vec < _ > = points . take (4) . collect () ; if points . len () == 4 { let fill = self . style . filled ; if points [0] . 1 > points [3] . 1 { points . swap (0 , 3) ; } let (l , r) = (self . width as i32 / 2 , self . width as i32 - self . width as i32 / 2 ,) ; backend . draw_line (points [0] , points [1] , & self . style) ? ; backend . draw_line (points [2] , points [3] , & self . style) ? ; points [0] . 0 -= l ; points [3] . 0 += r ; backend . draw_rect (points [0] , points [3] , & self . style , fill) ? ; } Ok (()) } }
};
}
