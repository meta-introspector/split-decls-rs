// Generated macro for impl_390 (impl)
macro_rules! Depcrate_calendarimpl_390 {
() => {
// Module: crate::calendar
// Provides: {"impl_390"}
// Dependencies: {}
impl < DS : DateStyler > Widget for & Monthly < '_ , DS > { fn render (self , area : Rect , buf : & mut Buffer) { self . block . as_ref () . render (area , buf) ; let inner = self . block . inner_if_some (area) ; self . render_monthly (inner , buf) ; } }
};
}
