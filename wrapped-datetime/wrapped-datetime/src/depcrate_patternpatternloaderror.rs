// Generated macro for PatternLoadError (enum)
macro_rules! Depcrate_patternPatternLoadError {
() => {
// Module: crate::pattern
// Provides: {"PatternLoadError"}
// Dependencies: {}
# [doc = " Error returned from [`FixedCalendarDateTimeNames`]'s pattern load methods."] # [derive (Debug , Clone , Copy , PartialEq , displaydoc :: Display)] # [non_exhaustive] pub enum PatternLoadError { # [doc = " A field conflicts with a previous field."] # [doc = ""] # [doc = " Fields conflict if they require the same type of data, for example the"] # [doc = " `EEE` and `EEEE` fields (short vs long weekday) conflict, or the `M`"] # [doc = " and `L` (format vs standalone month) conflict."] # [displaydoc ("A field {field:?} conflicts with a previously loaded field {previous_field:?}.")] ConflictingField { # [doc = " The field that was not able to be loaded."] field : ErrorField , # [doc = " The field that prevented the new field from being loaded."] previous_field : ErrorField , } , # [doc = " The field symbol is not supported in that length."] # [doc = ""] # [doc = " Some fields, such as `O` are not defined for all lengths (e.g. `OO`)."] # [displaydoc ("The field {0:?} symbol is not supported in that length.")] UnsupportedLength (ErrorField) , # [doc = " The specific formatter does not support this field."] # [doc = ""] # [doc = " This happens for example when trying to load a month field"] # [doc = " on a [`FixedCalendarDateTimeNames<Gregorian, ZoneFieldSet>`]."] # [displaydoc ("The specific formatter does not support the field {0:?}.")] FormatterTooSpecific (ErrorField) , # [doc = " An error arising from the [`data provider`](icu_provider) for loading names."] # [displaydoc ("Problem loading data for field {1:?}: {0}")] Data (icu_provider :: DataError , ErrorField) , }
};
}
