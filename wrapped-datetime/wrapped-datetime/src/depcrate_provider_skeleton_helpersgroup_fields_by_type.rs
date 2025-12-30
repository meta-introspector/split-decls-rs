// Generated macro for group_fields_by_type (function)
macro_rules! Depcrate_provider_skeleton_helpersgroup_fields_by_type {
() => {
// Module: crate::provider::skeleton::helpers
// Provides: {"group_fields_by_type"}
// Dependencies: {}
fn group_fields_by_type (fields : & [Field]) -> FieldsByType { let mut date = Vec :: new () ; let mut time = Vec :: new () ; for field in fields { match field . symbol { FieldSymbol :: Era | FieldSymbol :: Year (_) | FieldSymbol :: Month (_) | FieldSymbol :: Week (_) | FieldSymbol :: Day (_) | FieldSymbol :: Weekday (_) => date . push (* field) , FieldSymbol :: DayPeriod (_) | FieldSymbol :: Hour (_) | FieldSymbol :: Minute | FieldSymbol :: Second (_) | FieldSymbol :: TimeZone (_) | FieldSymbol :: DecimalSecond (_) => time . push (* field) , } ; } FieldsByType { date , time } }
};
}
