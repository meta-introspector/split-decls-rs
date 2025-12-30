// Generated macro for CandleStick (struct)
macro_rules! Depcrate_element_candlestickCandleStick {
() => {
// Module: crate::element::candlestick
// Provides: {"CandleStick"}
// Dependencies: {}
# [doc = " The candlestick data point element"] pub struct CandleStick < X , Y : PartialOrd > { style : ShapeStyle , width : u32 , points : [(X , Y) ; 4] , }
};
}
