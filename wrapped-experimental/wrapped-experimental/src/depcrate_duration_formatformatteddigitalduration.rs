// Generated macro for FormattedDigitalDuration (struct)
macro_rules! Depcrate_duration_formatFormattedDigitalDuration {
() => {
// Module: crate::duration::format
// Provides: {"FormattedDigitalDuration"}
// Dependencies: {}
struct FormattedDigitalDuration < 'l > { fmt : & 'l DurationFormatter , hours : Option < FormattedDecimal < 'l > > , add_hour_minute_separator : bool , minutes : Option < FormattedDecimal < 'l > > , add_minute_second_separator : bool , seconds : Option < FormattedDecimal < 'l > > , }
};
}
