// Generated macro for FormattedDuration (struct)
macro_rules! Depcrate_duration_formatFormattedDuration {
() => {
// Module: crate::duration::format
// Provides: {"FormattedDuration"}
// Dependencies: {}
# [doc = " The [`Writeable`] implementation that is returned by [`DurationFormatter::format`]. See"] # [doc = " the [`writeable`] crate for how to consume this."] pub struct FormattedDuration < 'l > { pub (crate) fmt : & 'l DurationFormatter , pub (crate) duration : & 'l Duration , }
};
}
