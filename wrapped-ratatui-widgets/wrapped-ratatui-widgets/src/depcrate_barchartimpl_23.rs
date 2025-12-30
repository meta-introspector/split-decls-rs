// Generated macro for impl_23 (impl)
macro_rules! Depcrate_barchartimpl_23 {
() => {
// Module: crate::barchart
// Provides: {"impl_23"}
// Dependencies: {}
impl Widget for & BarChart < '_ > { fn render (self , area : Rect , buf : & mut Buffer) { buf . set_style (area , self . style) ; self . block . as_ref () . render (area , buf) ; let inner = self . block . inner_if_some (area) ; if inner . is_empty () || self . data . is_empty () || self . bar_width == 0 { return ; } match self . direction { Direction :: Horizontal => self . render_horizontal (buf , inner) , Direction :: Vertical => self . render_vertical (buf , inner) , } } }
};
}
