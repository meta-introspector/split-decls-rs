// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl Widget for & PullRequestListWidget { fn render (self , area : Rect , buf : & mut Buffer) { let mut state = self . state . write () . unwrap () ; let loading_state = Line :: from (format ! ("{:?}" , state . loading_state)) . right_aligned () ; let block = Block :: bordered () . title ("Pull Requests") . title (loading_state) . title_bottom ("j/k to scroll, q to quit") ; let rows = state . pull_requests . iter () ; let widths = [Constraint :: Length (5) , Constraint :: Fill (1) , Constraint :: Max (49) ,] ; let table = Table :: new (rows , widths) . block (block) . highlight_spacing (HighlightSpacing :: Always) . highlight_symbol (">>") . row_highlight_style (Style :: new () . on_blue ()) ; StatefulWidget :: render (table , area , buf , & mut state . table_state) ; } }
};
}
