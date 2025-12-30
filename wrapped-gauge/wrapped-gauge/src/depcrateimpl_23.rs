// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl App { fn render_gauge1 (& self , area : Rect , buf : & mut Buffer) { let title = title_block ("Gauge with percentage") ; Gauge :: default () . block (title) . gauge_style (GAUGE1_COLOR) . percent (self . progress1) . render (area , buf) ; } fn render_gauge2 (& self , area : Rect , buf : & mut Buffer) { let title = title_block ("Gauge with ratio and custom label") ; let label = Span :: styled (format ! ("{:.1}/100" , self . progress2) , Style :: new () . italic () . bold () . fg (CUSTOM_LABEL_COLOR) ,) ; Gauge :: default () . block (title) . gauge_style (GAUGE2_COLOR) . ratio (self . progress2 / 100.0) . label (label) . render (area , buf) ; } fn render_gauge3 (& self , area : Rect , buf : & mut Buffer) { let title = title_block ("Gauge with ratio (no unicode)") ; let label = format ! ("{:.1}%" , self . progress3) ; Gauge :: default () . block (title) . gauge_style (GAUGE3_COLOR) . ratio (self . progress3 / 100.0) . label (label) . render (area , buf) ; } fn render_gauge4 (& self , area : Rect , buf : & mut Buffer) { let title = title_block ("Gauge with ratio (unicode)") ; let label = format ! ("{:.1}%" , self . progress3) ; Gauge :: default () . block (title) . gauge_style (GAUGE4_COLOR) . ratio (self . progress4 / 100.0) . label (label) . use_unicode (true) . render (area , buf) ; } }
};
}
