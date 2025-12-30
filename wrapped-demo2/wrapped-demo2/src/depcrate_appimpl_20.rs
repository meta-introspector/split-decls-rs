// Generated macro for impl_20 (impl)
macro_rules! Depcrate_appimpl_20 {
() => {
// Module: crate::app
// Provides: {"impl_20"}
// Dependencies: {}
# [doc = " Implement Widget for &App rather than for App as we would otherwise have to clone or copy the"] # [doc = " entire app state on every frame. For this example, the app state is small enough that it doesn't"] # [doc = " matter, but for larger apps this can be a significant performance improvement."] impl Widget for & App { fn render (self , area : Rect , buf : & mut Buffer) { let layout = Layout :: vertical ([Constraint :: Length (1) , Constraint :: Min (0) , Constraint :: Length (1) ,]) ; let [title_bar , tab , bottom_bar] = area . layout (& layout) ; Block :: new () . style (THEME . root) . render (area , buf) ; self . render_title_bar (title_bar , buf) ; self . render_selected_tab (tab , buf) ; App :: render_bottom_bar (bottom_bar , buf) ; } }
};
}
