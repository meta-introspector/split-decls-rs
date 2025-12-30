// Generated macro for test (module)
macro_rules! Depcrate_series_line_seriestest {
() => {
// Module: crate::series::line_series
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: prelude :: * ; # [test] fn test_line_series () { let drawing_area = create_mocked_drawing_area (200 , 200 , | m | { m . check_draw_path (| c , s , _path | { assert_eq ! (c , RED . to_rgba ()) ; assert_eq ! (s , 3) ; }) ; m . drop_check (| b | { assert_eq ! (b . num_draw_path_call , 8) ; assert_eq ! (b . draw_count , 27) ; }) ; }) ; let mut chart = ChartBuilder :: on (& drawing_area) . build_cartesian_2d (0 .. 100 , 0 .. 100) . expect ("Build chart error") ; chart . draw_series (LineSeries :: new ((0 .. 100) . map (| x | (x , x)) , Into :: < ShapeStyle > :: into (RED) . stroke_width (3) ,)) . expect ("Drawing Error") ; chart . draw_series (DashedLineSeries :: new ((0 ..= 50) . map (| x | (0 , x)) , 10 , 5 , Into :: < ShapeStyle > :: into (RED) . stroke_width (3) ,)) . expect ("Drawing Error") ; let mk_f = | c | Circle :: new (c , 3 , Into :: < ShapeStyle > :: into (RED) . filled ()) ; chart . draw_series (DottedLineSeries :: new ((0 ..= 50) . map (| x | (x , 0)) , 5 , 5 , mk_f)) . expect ("Drawing Error") ; } }
};
}
