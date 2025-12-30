// Generated macro for draw_mesh (function)
macro_rules! Depcrate_plots_stream_sparksdraw_mesh {
() => {
// Module: crate::plots::stream_sparks
// Provides: {"draw_mesh"}
// Dependencies: {}
fn draw_mesh < XT , YT , X , Y , DB : DrawingBackend > (colors : & PlotColors , chart : & mut ChartContext < DB , Cartesian2d < X , Y > > ,) where X : Ranged < ValueType = XT > + ValueFormatter < XT > , Y : Ranged < ValueType = YT > + ValueFormatter < YT > , { chart . configure_mesh () . disable_mesh () . x_labels (5) . y_labels (5) . axis_style (colors . axis) . label_style (chart_label_style (& colors . caption)) . draw () . unwrap () ; }
};
}
