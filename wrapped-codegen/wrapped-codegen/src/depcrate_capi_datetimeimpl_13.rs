// Generated macro for impl_13 (impl)
macro_rules! Depcrate_capi_datetimeimpl_13 {
() => {
// Module: crate::capi_datetime
// Provides: {"impl_13"}
// Dependencies: {}
impl FormatterKind { pub fn rust_type (self) -> & 'static str { match self . is_fixed_calendar { true => "FixedCalendarDateTimeFormatter" , false => "DateTimeFormatter" , } } pub fn rustlink (self) -> & 'static str { match (self . is_fixed_calendar , self . is_gregorian) { (true , true) => "FixedCalendarDateTimeFormatter" , (true , false) => "NoCalendarFormatter" , (false , _) => "DateTimeFormatter" , } } pub fn rustlink_doctype (self) -> & 'static str { match (self . is_fixed_calendar , self . is_gregorian) { (true , true) => "Struct" , (true , false) => "Typedef" , (false , _) => "Struct" , } } pub fn rustlink_doctype_fn (self) -> & 'static str { match (self . is_fixed_calendar , self . is_gregorian) { (true , true) => "FnInStruct" , (true , false) => "FnInTypedef" , (false , _) => "FnInStruct" , } } }
};
}
