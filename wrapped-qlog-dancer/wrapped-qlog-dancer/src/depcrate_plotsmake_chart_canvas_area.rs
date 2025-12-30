// Generated macro for make_chart_canvas_area (function)
macro_rules! Depcrate_plotsmake_chart_canvas_area {
() => {
// Module: crate::plots
// Provides: {"make_chart_canvas_area"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] pub fn make_chart_canvas_area (canvas_id : & str , colors : PlotColors , margin : ChartMargin ,) -> plotters :: drawing :: DrawingArea < CanvasBackend , plotters :: coord :: Shift > { let backend = CanvasBackend :: new (canvas_id) . unwrap_or_else (| | panic ! ("cannot find canvas {}" , canvas_id)) ; let area = backend . into_drawing_area () ; area . fill (& colors . fill) . unwrap () ; area . margin (margin . top , margin . bottom , margin . left , margin . right) }
};
}
