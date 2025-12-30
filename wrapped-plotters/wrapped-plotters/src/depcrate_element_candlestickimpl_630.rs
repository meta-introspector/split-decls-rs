// Generated macro for impl_630 (impl)
macro_rules! Depcrate_element_candlestickimpl_630 {
() => {
// Module: crate::element::candlestick
// Provides: {"impl_630"}
// Dependencies: {}
impl < X : Clone , Y : PartialOrd > CandleStick < X , Y > { # [doc = " Create a new candlestick element, which requires the Y coordinate can be compared"] # [doc = ""] # [doc = " - `x`: The x coordinate"] # [doc = " - `open`: The open value"] # [doc = " - `high`: The high value"] # [doc = " - `low`: The low value"] # [doc = " - `close`: The close value"] # [doc = " - `gain_style`: The style for gain"] # [doc = " - `loss_style`: The style for loss"] # [doc = " - `width`: The width"] # [doc = " - **returns** The newly created candlestick element"] # [doc = ""] # [doc = " ```rust"] # [doc = " use chrono::prelude::*;"] # [doc = " use plotters::prelude::*;"] # [doc = ""] # [doc = " let candlestick = CandleStick::new(Local::now(), 130.0600, 131.3700, 128.8300, 129.1500, &GREEN, &RED, 15);"] # [doc = " ```"] # [allow (clippy :: too_many_arguments)] pub fn new < GS : Into < ShapeStyle > , LS : Into < ShapeStyle > > (x : X , open : Y , high : Y , low : Y , close : Y , gain_style : GS , loss_style : LS , width : u32 ,) -> Self { Self { style : match open . partial_cmp (& close) { Some (Ordering :: Less) => gain_style . into () , _ => loss_style . into () , } , width , points : [(x . clone () , open) , (x . clone () , high) , (x . clone () , low) , (x , close) ,] , } } }
};
}
