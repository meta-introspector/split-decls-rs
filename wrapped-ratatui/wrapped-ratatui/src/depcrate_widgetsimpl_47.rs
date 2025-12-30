// Generated macro for impl_47 (impl)
macro_rules! Depcrate_widgetsimpl_47 {
() => {
// Module: crate::widgets
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (feature = "unstable-widget-ref")] impl FrameExt for ratatui_core :: terminal :: Frame < '_ > { fn render_widget_ref < W : WidgetRef > (& mut self , widget : W , area : Rect) { widget . render_ref (area , self . buffer_mut ()) ; } fn render_stateful_widget_ref < W > (& mut self , widget : W , area : Rect , state : & mut W :: State) where W : StatefulWidgetRef , { widget . render_ref (area , self . buffer_mut () , state) ; } }
};
}
