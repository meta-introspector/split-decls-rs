// Generated macro for impl_283 (impl)
macro_rules! Depcrate_pattern_namesimpl_283 {
() => {
// Module: crate::pattern::names
// Provides: {"impl_283"}
// Dependencies: {}
impl DayPeriodNameLength { pub (crate) fn to_attributes (self) -> & 'static DataMarkerAttributes { use marker_attrs :: Length ; let length = match self { DayPeriodNameLength :: Abbreviated => Length :: Abbr , DayPeriodNameLength :: Wide => Length :: Wide , DayPeriodNameLength :: Narrow => Length :: Narrow , } ; marker_attrs :: name_attr_for (marker_attrs :: Context :: Format , length) } pub (crate) fn from_field (field_symbol : fields :: DayPeriod , field_length : FieldLength ,) -> Option < Self > { use fields :: DayPeriod ; let field_symbol = match field_symbol { DayPeriod :: NoonMidnight => DayPeriod :: AmPm , other => other , } ; let field_length = field_length . numeric_to_abbr () ; match (field_symbol , field_length) { (DayPeriod :: AmPm , FieldLength :: Three) => Some (DayPeriodNameLength :: Abbreviated) , (DayPeriod :: AmPm , FieldLength :: Four) => Some (DayPeriodNameLength :: Wide) , (DayPeriod :: AmPm , FieldLength :: Five) => Some (DayPeriodNameLength :: Narrow) , _ => None , } } # [doc = " Returns an [`ErrorField`] sufficient for error reporting."] pub (crate) fn to_approximate_error_field (self) -> ErrorField { let field_symbol = fields :: DayPeriod :: AmPm ; let field_length = match self { DayPeriodNameLength :: Abbreviated => FieldLength :: Three , DayPeriodNameLength :: Wide => FieldLength :: Four , DayPeriodNameLength :: Narrow => FieldLength :: Five , } ; ErrorField (fields :: Field { symbol : FieldSymbol :: DayPeriod (field_symbol) , length : field_length , }) } }
};
}
