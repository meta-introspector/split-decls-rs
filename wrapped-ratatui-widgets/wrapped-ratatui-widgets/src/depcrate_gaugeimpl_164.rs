// Generated macro for impl_164 (impl)
macro_rules! Depcrate_gaugeimpl_164 {
() => {
// Module: crate::gauge
// Provides: {"impl_164"}
// Dependencies: {}
impl Gauge < '_ > { fn render_gauge (& self , gauge_area : Rect , buf : & mut Buffer) { if gauge_area . is_empty () { return ; } buf . set_style (gauge_area , self . gauge_style) ; let default_label = Span :: raw (format ! ("{}%" , f64 :: round (self . ratio * 100.0))) ; let label = self . label . as_ref () . unwrap_or (& default_label) ; let clamped_label_width = gauge_area . width . min (label . width () as u16) ; let label_col = gauge_area . left () + (gauge_area . width - clamped_label_width) / 2 ; let label_row = gauge_area . top () + gauge_area . height / 2 ; let filled_width = f64 :: from (gauge_area . width) * self . ratio ; let end = if self . use_unicode { gauge_area . left () + filled_width . floor () as u16 } else { gauge_area . left () + filled_width . round () as u16 } ; for y in gauge_area . top () .. gauge_area . bottom () { for x in gauge_area . left () .. end { if x < label_col || x > label_col + clamped_label_width || y != label_row { buf [(x , y)] . set_symbol (symbols :: block :: FULL) . set_fg (self . gauge_style . fg . unwrap_or (Color :: Reset)) . set_bg (self . gauge_style . bg . unwrap_or (Color :: Reset)) ; } else { buf [(x , y)] . set_symbol (" ") . set_fg (self . gauge_style . bg . unwrap_or (Color :: Reset)) . set_bg (self . gauge_style . fg . unwrap_or (Color :: Reset)) ; } } if self . use_unicode && self . ratio < 1.0 { buf [(end , y)] . set_symbol (get_unicode_block (filled_width % 1.0)) ; } } buf . set_span (label_col , label_row , label , clamped_label_width) ; } }
};
}
