// Generated macro for draw_third_tab (function)
macro_rules! Depcrate_uidraw_third_tab {
() => {
// Module: crate::ui
// Provides: {"draw_third_tab"}
// Dependencies: {}
fn draw_third_tab (frame : & mut Frame , _app : & mut App , area : Rect) { let chunks = Layout :: horizontal ([Constraint :: Ratio (1 , 2) , Constraint :: Ratio (1 , 2)]) . split (area) ; let colors = [Color :: Reset , Color :: Black , Color :: Red , Color :: Green , Color :: Yellow , Color :: Blue , Color :: Magenta , Color :: Cyan , Color :: Gray , Color :: DarkGray , Color :: LightRed , Color :: LightGreen , Color :: LightYellow , Color :: LightBlue , Color :: LightMagenta , Color :: LightCyan , Color :: White ,] ; let items : Vec < Row > = colors . iter () . map (| c | { let cells = vec ! [Cell :: from (Span :: raw (format ! ("{c:?}: "))) , Cell :: from (Span :: styled ("Foreground" , Style :: default () . fg (* c))) , Cell :: from (Span :: styled ("Background" , Style :: default () . bg (* c))) ,] ; Row :: new (cells) }) . collect () ; let table = Table :: new (items , [Constraint :: Ratio (1 , 3) , Constraint :: Ratio (1 , 3) , Constraint :: Ratio (1 , 3) ,] ,) . block (Block :: bordered () . title ("Colors")) ; frame . render_widget (table , chunks [0]) ; }
};
}
