// Generated macro for draw_cc_updates (function)
macro_rules! Depcrate_plots_congestion_controldraw_cc_updates {
() => {
// Module: crate::plots::congestion_control
// Provides: {"draw_cc_updates"}
// Dependencies: {}
fn draw_cc_updates < DB : DrawingBackend > (data : & [(f32 , u64 , String)] , y_range : std :: ops :: Range < u64 > , y_range_extended : std :: ops :: Range < u64 > , congestion_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { let my_label = | x : f32 , y : u64 , name : & str | { let color = cc_state_to_color (name) ; let text_width = name . len () as i32 * 6 ; let text_height = 12 ; return EmptyElement :: at ((x , y)) + Rectangle :: new ([(1 , - 2) , (1 + text_width , text_height)] , WHITE . mix (0.7) . filled ()) + Text :: new (name . to_owned () , (1 , 0) , ("sans-serif" , 12.0) . into_font () . color (& color) ,) ; } ; let mut woggle = vec ! [0] ; for i in 1 .. 5 { woggle . push ((y_range . end / 10) * i) ; } let iter = data . iter () . rev () ; for cc_state in iter { let color = cc_state_to_color (& cc_state . 2) ; let x = cc_state . 0 ; let line_coords = [(x , y_range_extended . start) , (x , y_range_extended . end)] ; congestion_chart . draw_series (LineSeries :: new (line_coords , color)) . unwrap () ; let area = congestion_chart . plotting_area () ; let y = y_range . end + woggle [0] ; area . draw (& my_label (x , y , & cc_state . 2)) . unwrap () ; woggle . rotate_left (1) ; } }
};
}
