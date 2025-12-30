// Generated macro for DurationFormatter (struct)
macro_rules! Depcrate_duration_formatterDurationFormatter {
() => {
// Module: crate::duration::formatter
// Provides: {"DurationFormatter"}
// Dependencies: {}
# [doc = " A formatter for [`Duration`](crate::duration::Duration)s."] # [doc = ""] # [doc = " [`DurationFormatter`] supports:"] # [doc = ""] # [doc = " 1. Rendering with different styles for each unit"] # [doc = " 2. Digital formatting style"] # [doc = " 3. Positive and negative duraitons"] # [doc = ""] # [doc = " Read more about the options in the [`options`](super::options) module."] # [doc = ""] # [doc = " See the crate-level documentation for examples."] pub struct DurationFormatter { # [doc = " Options for configuring the formatter."] pub (crate) options : ValidatedDurationFormatterOptions , pub (crate) digital : DataPayload < provider :: DigitalDurationDataV1 > , pub (crate) unit : DurationUnitFormatter , pub (crate) list : ListFormatter , pub (crate) fdf : DecimalFormatter , }
};
}
