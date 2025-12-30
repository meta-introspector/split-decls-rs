// Generated macro for make_chart_bitmap_area (function)
macro_rules! Depcrate_plotsmake_chart_bitmap_area {
() => {
// Module: crate::plots
// Provides: {"make_chart_bitmap_area"}
// Dependencies: {}
# [cfg (not (target_arch = "wasm32"))] pub fn make_chart_bitmap_area (path : & str , size : ChartSize , colors : PlotColors , margin : ChartMargin ,) -> DrawingArea < plotters :: prelude :: BitMapBackend < '_ > , plotters :: coord :: Shift > { let backend = BitMapBackend :: new (path , (size . width , size . height)) ; let area = backend . into_drawing_area () ; area . fill (& colors . fill) . unwrap () ; area . margin (margin . top , margin . bottom , margin . left , margin . right) }
};
}
