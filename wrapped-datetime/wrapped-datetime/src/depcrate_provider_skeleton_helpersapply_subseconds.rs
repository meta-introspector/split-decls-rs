// Generated macro for apply_subseconds (function)
macro_rules! Depcrate_provider_skeleton_helpersapply_subseconds {
() => {
// Module: crate::provider::skeleton::helpers
// Provides: {"apply_subseconds"}
// Dependencies: {}
# [doc = " Alters given Pattern so that it will have a fractional second field if it was requested."] # [doc = ""] # [doc = " If the requested skeleton included both seconds and fractional seconds and the dateFormatItem"] # [doc = " skeleton included seconds but not fractional seconds, then the seconds field of the corresponding"] # [doc = " pattern should be adjusted by appending the locale’s decimal separator, followed by the sequence"] # [doc = " of ‘S’ characters from the requested skeleton."] # [doc = " (see <https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons>)"] fn apply_subseconds (pattern : & mut runtime :: Pattern , subseconds : Option < SubsecondDigits >) { if let Some (subseconds) = subseconds { let mut items = pattern . items . to_vec () ; for item in items . iter_mut () { if let PatternItem :: Field (ref mut field @ Field { symbol : FieldSymbol :: Second (fields :: Second :: Second) | FieldSymbol :: DecimalSecond (_) , .. } ,) = item { field . symbol = FieldSymbol :: from_subsecond_digits (subseconds) ; } ; } * pattern = runtime :: Pattern :: from (items) ; pattern . metadata . set_time_granularity (TimeGranularity :: Nanoseconds) ; } }
};
}
