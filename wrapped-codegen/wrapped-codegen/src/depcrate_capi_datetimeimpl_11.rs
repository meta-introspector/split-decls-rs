// Generated macro for impl_11 (impl)
macro_rules! Depcrate_capi_datetimeimpl_11 {
() => {
// Module: crate::capi_datetime
// Provides: {"impl_11"}
// Dependencies: {}
impl FormatterFlavor { pub fn camel (self) -> & 'static str { match self { FormatterFlavor :: Date => "Date" , FormatterFlavor :: Time => "Time" , FormatterFlavor :: Zone => "Zone" , FormatterFlavor :: DateTime => "DateTime" , } } pub fn lower (self) -> & 'static str { match self { FormatterFlavor :: Date => "date" , FormatterFlavor :: Time => "time" , FormatterFlavor :: Zone => "zone" , FormatterFlavor :: DateTime => "datetime" , } } pub fn formatter_kinds (self) -> & 'static [FormatterKind] { match self { FormatterFlavor :: Date | FormatterFlavor :: DateTime => & [FormatterKind { is_fixed_calendar : false , is_gregorian : false , } , FormatterKind { is_fixed_calendar : true , is_gregorian : true , } ,] , FormatterFlavor :: Time | FormatterFlavor :: Zone => & [FormatterKind { is_fixed_calendar : true , is_gregorian : false , }] , } } pub fn field_set (self) -> & 'static str { match self { FormatterFlavor :: Date => "DateFieldSet" , FormatterFlavor :: Time => "TimeFieldSet" , FormatterFlavor :: Zone => "ZoneFieldSet" , FormatterFlavor :: DateTime => "CompositeDateTimeFieldSet" , } } pub fn has_date (self) -> bool { matches ! (self , FormatterFlavor :: Date | FormatterFlavor :: DateTime) } pub fn has_time (self) -> bool { matches ! (self , FormatterFlavor :: Time | FormatterFlavor :: DateTime) } pub fn is_time_only (self) -> bool { matches ! (self , FormatterFlavor :: Time) } pub fn is_zone_only (self) -> bool { matches ! (self , FormatterFlavor :: Zone) } }
};
}
