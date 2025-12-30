// Generated macro for impl_264 (impl)
macro_rules! Depcrate_scrollbarimpl_264 {
() => {
// Module: crate::scrollbar
// Provides: {"impl_264"}
// Dependencies: {}
impl StatefulWidget for Scrollbar < '_ > { type State = ScrollbarState ; fn render (self , area : Rect , buf : & mut Buffer , state : & mut Self :: State) { if state . content_length == 0 || self . track_length_excluding_arrow_heads (area) == 0 { return ; } if let Some (area) = self . scrollbar_area (area) { let areas = area . columns () . flat_map (Rect :: rows) ; let bar_symbols = self . bar_symbols (area , state) ; for (area , bar) in areas . zip (bar_symbols) { if let Some ((symbol , style)) = bar { buf . set_string (area . x , area . y , symbol , style) ; } } } } }
};
}
