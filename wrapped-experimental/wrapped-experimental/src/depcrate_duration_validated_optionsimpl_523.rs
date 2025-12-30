// Generated macro for impl_523 (impl)
macro_rules! Depcrate_duration_validated_optionsimpl_523 {
() => {
// Module: crate::duration::validated_options
// Provides: {"impl_523"}
// Dependencies: {}
impl Unit { # [doc = " Returns the default digital style for the unit."] pub (crate) fn digital_default (& self) -> FieldStyle { match self { Unit :: Year => YearStyle :: Short . into () , Unit :: Month => MonthStyle :: Short . into () , Unit :: Week => WeekStyle :: Short . into () , Unit :: Day => DayStyle :: Short . into () , Unit :: Hour => HourStyle :: Numeric . into () , Unit :: Minute => MinuteStyle :: Numeric . into () , Unit :: Second => SecondStyle :: Numeric . into () , Unit :: Millisecond => MilliSecondStyle :: Numeric . into () , Unit :: Microsecond => MicroSecondStyle :: Numeric . into () , Unit :: Nanosecond => NanoSecondStyle :: Numeric . into () , } } pub (crate) const fn as_unit_formatter_name (& self) -> & 'static str { match self { Unit :: Year => "year" , Unit :: Month => "month" , Unit :: Week => "week" , Unit :: Day => "day" , Unit :: Hour => "hour" , Unit :: Minute => "minute" , Unit :: Second => "second" , Unit :: Millisecond => "millisecond" , Unit :: Microsecond => "microsecond" , Unit :: Nanosecond => "nanosecond" , } } }
};
}
