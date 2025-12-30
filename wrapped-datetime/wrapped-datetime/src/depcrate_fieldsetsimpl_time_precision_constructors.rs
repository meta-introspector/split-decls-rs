// Generated macro for impl_time_precision_constructors (macro)
macro_rules! Depcrate_fieldsetsimpl_time_precision_constructors {
() => {
// Module: crate::fieldsets
// Provides: {"impl_time_precision_constructors"}
// Dependencies: {}
macro_rules ! impl_time_precision_constructors { ($ time_type : ident ,) => { impl $ time_type { # [doc = concat ! ("Creates a " , stringify ! ($ type) , " that formats hours and minutes with the default length.")] pub fn hm () -> Self { Self :: for_length (Default :: default ()) . with_time_precision (TimePrecision :: Minute) } # [doc = concat ! ("Creates a " , stringify ! ($ type) , " that formats hours, minutes, and seconds with the default length.")] pub fn hms () -> Self { Self :: for_length (Default :: default ()) . with_time_precision (TimePrecision :: Second) } # [doc = concat ! ("Creates a " , stringify ! ($ type) , " that formats hours, minutes, seconds, and subseconds with the default length.")] pub fn hmss (subsecond_digits : SubsecondDigits) -> Self { Self :: for_length (Default :: default ()) . with_time_precision (TimePrecision :: Subsecond (subsecond_digits)) } } } ; }
};
}
