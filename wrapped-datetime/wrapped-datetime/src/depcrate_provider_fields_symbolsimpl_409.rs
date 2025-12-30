// Generated macro for impl_409 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_409 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_409"}
// Dependencies: {}
impl From < FieldSymbol > for char { fn from (symbol : FieldSymbol) -> Self { match symbol { FieldSymbol :: Era => 'G' , FieldSymbol :: Year (year) => year . into () , FieldSymbol :: Month (month) => month . into () , FieldSymbol :: Week (week) => week . into () , FieldSymbol :: Day (day) => day . into () , FieldSymbol :: Weekday (weekday) => weekday . into () , FieldSymbol :: DayPeriod (dayperiod) => dayperiod . into () , FieldSymbol :: Hour (hour) => hour . into () , FieldSymbol :: Minute => 'm' , FieldSymbol :: Second (second) => second . into () , FieldSymbol :: TimeZone (time_zone) => time_zone . into () , FieldSymbol :: DecimalSecond (_) => 's' , } } }
};
}
