// Generated macro for DurationFormatterOptionsError (enum)
macro_rules! Depcrate_duration_validated_optionsDurationFormatterOptionsError {
() => {
// Module: crate::duration::validated_options
// Provides: {"DurationFormatterOptionsError"}
// Dependencies: {}
# [doc = " Error type for [`DurationFormatterOptions`] validation."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq , displaydoc :: Display)] pub enum DurationFormatterOptionsError { # [doc = " A unit field is set to [`FieldDisplay::Always`] and the style is set to [`FieldStyle::Fractional`]."] # [displaydoc ("A unit field is set to Always and the style is set to Fractional")] DisplayAlwaysFractional , # [doc = " A previous unit's style is [`FieldStyle::Fractional`], but the following unit's style is not [`FieldStyle::Fractional`]."] # [displaydoc ("A previous unit's style is Fractional, but the following unit's style is not Fractional")] PreviousFractional , # [doc = " A previous unit's style is set to [`FieldStyle::Numeric`] or [`FieldStyle::TwoDigit`] and the following unit's style is not"] # [doc = " [`FieldStyle::Fractional`], [`FieldStyle::Numeric`], or [`FieldStyle::TwoDigit`]."] # [displaydoc ("A previous unit's style is set to Numeric or TwoDigit and the following unit's style is not Fractional, Numeric, or TwoDigit")] PreviousNumeric , # [doc = " The number of fractional digits is out of acceptable range. See [`FractionalDigits::Fixed`]."] # [displaydoc ("The number of fractional digits is out of acceptable range")] FractionalDigitsOutOfRange , }
};
}
