// Generated macro for render (function)
macro_rules! Depcrate_uirender {
() => {
// Module: crate::ui
// Provides: {"render"}
// Dependencies: {}
pub fn render (frame : & mut Frame , app : & mut App) { let chunks = Layout :: vertical ([Constraint :: Length (3) , Constraint :: Min (0)]) . split (frame . area ()) ; let tabs = app . tabs . titles . iter () . map (| t | text :: Line :: from (Span :: styled (* t , Style :: default () . fg (Color :: Green)))) . collect :: < Tabs > () . block (Block :: bordered () . title (app . title)) . highlight_style (Style :: default () . fg (Color :: Yellow)) . select (app . tabs . index) ; frame . render_widget (tabs , chunks [0]) ; match app . tabs . index { 0 => draw_first_tab (frame , app , chunks [1]) , 1 => draw_second_tab (frame , app , chunks [1]) , 2 => draw_third_tab (frame , app , chunks [1]) , _ => { } } ; }
};
}
