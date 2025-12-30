// Generated macro for impl_631 (impl)
macro_rules! Depcrate_element_candlestickimpl_631 {
() => {
// Module: crate::element::candlestick
// Provides: {"impl_631"}
// Dependencies: {}
impl < 'a , X : 'a , Y : PartialOrd + 'a > PointCollection < 'a , (X , Y) > for & 'a CandleStick < X , Y > { type Point = & 'a (X , Y) ; type IntoIter = & 'a [(X , Y)] ; fn point_iter (self) -> & 'a [(X , Y)] { & self . points } }
};
}
