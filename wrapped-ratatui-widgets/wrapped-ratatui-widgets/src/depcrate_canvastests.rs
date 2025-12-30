// Generated macro for tests (module)
macro_rules! Depcrate_canvastests {
() => {
// Module: crate::canvas
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use indoc :: indoc ; use ratatui_core :: buffer :: Cell ; use super :: * ; fn test_marker (marker : Marker , expected : & str) { let area = Rect :: new (0 , 0 , 5 , 5) ; let mut buf = Buffer :: filled (area , Cell :: new ("x")) ; let horizontal_line = Line { x1 : 0.0 , y1 : 0.0 , x2 : 10.0 , y2 : 0.0 , color : Color :: Reset , } ; let vertical_line = Line { x1 : 0.0 , y1 : 0.0 , x2 : 0.0 , y2 : 10.0 , color : Color :: Reset , } ; Canvas :: default () . marker (marker) . paint (| ctx | { ctx . draw (& vertical_line) ; ctx . draw (& horizontal_line) ; }) . x_bounds ([0.0 , 10.0]) . y_bounds ([0.0 , 10.0]) . render (area , & mut buf) ; assert_eq ! (buf , Buffer :: with_lines (expected . lines ())) ; } # [test] fn test_bar_marker () { test_marker (Marker :: Bar , indoc ! ("
                ▄xxxx
                ▄xxxx
                ▄xxxx
                ▄xxxx
                ▄▄▄▄▄") ,) ; } # [test] fn test_block_marker () { test_marker (Marker :: Block , indoc ! ("
                █xxxx
                █xxxx
                █xxxx
                █xxxx
                █████") ,) ; } # [test] fn test_braille_marker () { test_marker (Marker :: Braille , indoc ! ("
                ⡇xxxx
                ⡇xxxx
                ⡇xxxx
                ⡇xxxx
                ⣇⣀⣀⣀⣀") ,) ; } # [test] fn test_dot_marker () { test_marker (Marker :: Dot , indoc ! ("
                •xxxx
                •xxxx
                •xxxx
                •xxxx
                •••••") ,) ; } # [test] fn check_canvas_paint_max () { let mut b_grid = BrailleGrid :: new (u16 :: MAX , 2) ; let mut c_grid = CharGrid :: new (u16 :: MAX , 2 , 'd') ; let max = u16 :: MAX as usize ; b_grid . paint (0 , 0 , Color :: Red) ; b_grid . paint (0 , max , Color :: Red) ; b_grid . paint (max , 0 , Color :: Red) ; b_grid . paint (max , max , Color :: Red) ; c_grid . paint (0 , 0 , Color :: Red) ; c_grid . paint (0 , max , Color :: Red) ; c_grid . paint (max , 0 , Color :: Red) ; c_grid . paint (max , max , Color :: Red) ; } # [test] fn check_canvas_paint_overflow () { let mut b_grid = BrailleGrid :: new (u16 :: MAX , 3) ; let mut c_grid = CharGrid :: new (u16 :: MAX , 3 , 'd') ; let max = u16 :: MAX as usize + 10 ; b_grid . paint (max , max , Color :: Red) ; c_grid . paint (max , max , Color :: Red) ; b_grid . paint (usize :: MAX , usize :: MAX , Color :: Red) ; c_grid . paint (usize :: MAX , usize :: MAX , Color :: Red) ; } # [test] fn render_in_minimal_buffer () { let mut buffer = Buffer :: empty (Rect :: new (0 , 0 , 1 , 1)) ; let canvas = Canvas :: default () . x_bounds ([0.0 , 10.0]) . y_bounds ([0.0 , 10.0]) . paint (| _ctx | { }) ; canvas . render (buffer . area , & mut buffer) ; assert_eq ! (buffer , Buffer :: with_lines ([" "])) ; } # [test] fn render_in_zero_size_buffer () { let mut buffer = Buffer :: empty (Rect :: ZERO) ; let canvas = Canvas :: default () . x_bounds ([0.0 , 10.0]) . y_bounds ([0.0 , 10.0]) . paint (| _ctx | { }) ; canvas . render (buffer . area , & mut buffer) ; } }
};
}
