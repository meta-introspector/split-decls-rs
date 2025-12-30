// Generated macro for draw_stream_frames_line (function)
macro_rules! Depcrate_plots_stream_sparksdraw_stream_frames_line {
() => {
// Module: crate::plots::stream_sparks
// Provides: {"draw_stream_frames_line"}
// Dependencies: {}
fn draw_stream_frames_line < DB : DrawingBackend > (stream_frames : & [(f32 , u64)] , abs_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > , rel_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { abs_chart . draw_series (LineSeries :: new (stream_frames . to_vec () , PURPLE_500)) . unwrap () ; rel_chart . draw_series (LineSeries :: new (stream_frames . to_vec () , PURPLE_500)) . unwrap () ; let circles : Vec < Circle < (f32 , u64) , i32 > > = stream_frames . iter () . map (| point | Circle :: new (* point , 2 , PURPLE_500)) . collect () ; rel_chart . draw_series (circles) . unwrap () ; }
};
}
