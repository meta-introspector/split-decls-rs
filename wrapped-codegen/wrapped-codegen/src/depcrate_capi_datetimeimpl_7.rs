// Generated macro for impl_7 (impl)
macro_rules! Depcrate_capi_datetimeimpl_7 {
() => {
// Module: crate::capi_datetime
// Provides: {"impl_7"}
// Dependencies: {}
impl ConsumedOptions { fn from_builder (builder : FieldSetBuilder) -> Option < Self > { match builder . build_composite () { Ok (_) => Some (ConsumedOptions { length : true , alignment : true , year_style : true , }) , Err (BuilderError :: SuperfluousOptions (options)) => Some (ConsumedOptions { length : options . length . is_none () , alignment : options . alignment . is_none () , year_style : options . year_style . is_none () , }) , Err (BuilderError :: InvalidDateFields) => None , Err (e) => panic ! ("unexpected error: {e}") , } } }
};
}
