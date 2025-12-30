// Generated macro for impl_461 (impl)
macro_rules! Depcrate_provider_fieldsimpl_461 {
() => {
// Module: crate::provider::fields
// Provides: {"impl_461"}
// Dependencies: {}
impl Field { # [cfg (feature = "datagen")] pub (crate) fn get_length_type (self) -> TextOrNumeric { match self . symbol { FieldSymbol :: Era => TextOrNumeric :: Text , FieldSymbol :: Year (year) => year . get_length_type (self . length) , FieldSymbol :: Month (month) => month . get_length_type (self . length) , FieldSymbol :: Week (week) => week . get_length_type (self . length) , FieldSymbol :: Day (day) => day . get_length_type (self . length) , FieldSymbol :: Weekday (weekday) => weekday . get_length_type (self . length) , FieldSymbol :: DayPeriod (day_period) => day_period . get_length_type (self . length) , FieldSymbol :: Hour (hour) => hour . get_length_type (self . length) , FieldSymbol :: Minute => TextOrNumeric :: Numeric , FieldSymbol :: Second (second) => second . get_length_type (self . length) , FieldSymbol :: TimeZone (zone) => zone . get_length_type (self . length) , FieldSymbol :: DecimalSecond (_) => TextOrNumeric :: Numeric , } } }
};
}
