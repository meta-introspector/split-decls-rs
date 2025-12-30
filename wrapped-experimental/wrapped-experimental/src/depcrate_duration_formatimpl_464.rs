// Generated macro for impl_464 (impl)
macro_rules! Depcrate_duration_formatimpl_464 {
() => {
// Module: crate::duration::format
// Provides: {"impl_464"}
// Dependencies: {}
impl DigitalDuration { fn format < 'l > (& 'l self , fmt : & 'l DurationFormatter) -> FormattedDigitalDuration < 'l > { FormattedDigitalDuration { fmt , hours : self . hours . as_ref () . map (| h | fmt . fdf . format (h)) , add_hour_minute_separator : self . add_hour_minute_separator , minutes : self . minutes . as_ref () . map (| m | fmt . fdf . format (m)) , add_minute_second_separator : self . add_minute_second_separator , seconds : self . seconds . as_ref () . map (| s | fmt . fdf . format (s)) , } } }
};
}
