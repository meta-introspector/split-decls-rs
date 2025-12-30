// Generated macro for render_line_chart (function)
macro_rules! Depcraterender_line_chart {
() => {
// Module: crate
// Provides: {"render_line_chart"}
// Dependencies: {}
fn render_line_chart (frame : & mut Frame , area : Rect) { let datasets = vec ! [Dataset :: default () . name ("Line from only 2 points" . italic ()) . marker (symbols :: Marker :: Braille) . style (Style :: default () . fg (Color :: Yellow)) . graph_type (GraphType :: Line) . data (& [(1. , 1.) , (4. , 4.)]) ,] ; let chart = Chart :: new (datasets) . block (Block :: bordered () . title (Line :: from ("Line chart") . cyan () . bold () . centered ())) . x_axis (Axis :: default () . title ("X Axis") . style (Style :: default () . gray ()) . bounds ([0.0 , 5.0]) . labels (["0" . bold () , "2.5" . into () , "5.0" . bold ()]) ,) . y_axis (Axis :: default () . title ("Y Axis") . style (Style :: default () . gray ()) . bounds ([0.0 , 5.0]) . labels (["0" . bold () , "2.5" . into () , "5.0" . bold ()]) ,) . legend_position (Some (LegendPosition :: TopLeft)) . hidden_legend_constraints ((Constraint :: Ratio (1 , 2) , Constraint :: Ratio (1 , 2))) ; frame . render_widget (chart , area) ; }
};
}
