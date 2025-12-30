// Generated macro for impl_170 (impl)
macro_rules! Depcrate_gaugeimpl_170 {
() => {
// Module: crate::gauge
// Provides: {"impl_170"}
// Dependencies: {}
impl Widget for & LineGauge < '_ > { fn render (self , area : Rect , buf : & mut Buffer) { buf . set_style (area , self . style) ; self . block . as_ref () . render (area , buf) ; let gauge_area = self . block . inner_if_some (area) ; if gauge_area . is_empty () { return ; } let ratio = self . ratio ; let default_label = Line :: from (format ! ("{:3.0}%" , ratio * 100.0)) ; let label = self . label . as_ref () . unwrap_or (& default_label) ; let (col , row) = buf . set_line (gauge_area . left () , gauge_area . top () , label , gauge_area . width) ; let start = col + 1 ; if start >= gauge_area . right () { return ; } let end = start + (f64 :: from (gauge_area . right () . saturating_sub (start)) * self . ratio) . floor () as u16 ; for col in start .. end { buf [(col , row)] . set_symbol (self . filled_symbol) . set_style (self . filled_style) ; } for col in end .. gauge_area . right () { buf [(col , row)] . set_symbol (self . unfilled_symbol) . set_style (self . unfilled_style) ; } } }
};
}
