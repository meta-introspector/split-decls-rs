// Generated macro for Candlesticks (struct)
macro_rules! Depcrate_candlestickCandlesticks {
() => {
// Module: crate::candlestick
// Provides: {"Candlesticks"}
// Dependencies: {}
# [doc = " A candlestick consists of a box and two whiskers that extend beyond the box"] pub struct Candlesticks < X , WM , BM , BH , WH > { # [doc = " X coordinate of the candlestick"] pub x : X , # [doc = " Y coordinate of the end point of the bottom whisker"] pub whisker_min : WM , # [doc = " Y coordinate of the bottom of the box"] pub box_min : BM , # [doc = " Y coordinate of the top of the box"] pub box_high : BH , # [doc = " Y coordinate of the end point of the top whisker"] pub whisker_high : WH , }
};
}
