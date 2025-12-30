// Generated macro for impl_163 (impl)
macro_rules! Depcrate_gaugeimpl_163 {
() => {
// Module: crate::gauge
// Provides: {"impl_163"}
// Dependencies: {}
impl Widget for & Gauge < '_ > { fn render (self , area : Rect , buf : & mut Buffer) { buf . set_style (area , self . style) ; self . block . as_ref () . render (area , buf) ; let inner = self . block . inner_if_some (area) ; self . render_gauge (inner , buf) ; } }
};
}
