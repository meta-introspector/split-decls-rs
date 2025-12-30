// Generated macro for impl_741 (impl)
macro_rules! Depcrate_output_render_timesimpl_741 {
() => {
// Module: crate::output::render::times
// Provides: {"impl_741"}
// Dependencies: {}
impl Render for Option < NaiveDateTime > { fn render (self , style : Style , time_offset : FixedOffset , time_format : TimeFormat) -> TextCell { let datestamp = if let Some (time) = self { time_format . format (& DateTime :: < FixedOffset > :: from_naive_utc_and_offset (time , time_offset ,)) } else { String :: from ("-") } ; TextCell :: paint (style , datestamp) } }
};
}
