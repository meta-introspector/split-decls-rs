// Generated macro for render_scatter (function)
macro_rules! Depcraterender_scatter {
() => {
// Module: crate
// Provides: {"render_scatter"}
// Dependencies: {}
fn render_scatter (frame : & mut Frame , area : Rect) { let datasets = vec ! [Dataset :: default () . name ("Heavy") . marker (Marker :: Dot) . graph_type (GraphType :: Scatter) . style (Style :: new () . yellow ()) . data (& HEAVY_PAYLOAD_DATA) , Dataset :: default () . name ("Medium" . underlined ()) . marker (Marker :: Braille) . graph_type (GraphType :: Scatter) . style (Style :: new () . magenta ()) . data (& MEDIUM_PAYLOAD_DATA) , Dataset :: default () . name ("Small") . marker (Marker :: Dot) . graph_type (GraphType :: Scatter) . style (Style :: new () . cyan ()) . data (& SMALL_PAYLOAD_DATA) ,] ; let chart = Chart :: new (datasets) . block (Block :: bordered () . title (Line :: from ("Scatter chart") . cyan () . bold () . centered ())) . x_axis (Axis :: default () . title ("Year") . bounds ([1960. , 2020.]) . style (Style :: default () . fg (Color :: Gray)) . labels (["1960" , "1990" , "2020"]) ,) . y_axis (Axis :: default () . title ("Cost") . bounds ([0. , 75000.]) . style (Style :: default () . fg (Color :: Gray)) . labels (["0" , "37 500" , "75 000"]) ,) . hidden_legend_constraints ((Constraint :: Ratio (1 , 2) , Constraint :: Ratio (1 , 2))) ; frame . render_widget (chart , area) ; }
};
}
