// Generated macro for draw_mesh (function)
macro_rules! Depcrate_plotsdraw_mesh {
() => {
// Module: crate::plots
// Provides: {"draw_mesh"}
// Dependencies: {}
fn draw_mesh < XT , YT , X , Y , DB : DrawingBackend > (colors : & PlotColors , x_desc : & str , y_desc : & str , draw_minor_lines : bool , chart : & mut ChartContext < DB , Cartesian2d < X , Y > > ,) where X : Ranged < ValueType = XT > + ValueFormatter < XT > , Y : Ranged < ValueType = YT > + ValueFormatter < YT > , { let mut mesh = chart . configure_mesh () ; mesh . axis_style (colors . axis) . x_desc (x_desc) . y_desc (y_desc) . bold_line_style (colors . bold_line . mix (0.5)) ; if ! draw_minor_lines { mesh . light_line_style (colors . bold_line . mix (0.0)) ; } else { mesh . light_line_style (colors . light_line . mix (0.2)) ; } mesh . label_style (chart_label_style (& colors . caption)) . draw () . unwrap () ; }
};
}
