// Generated macro for for_datetime (function)
macro_rules! Depcrate_util_round_incrementfor_datetime {
() => {
// Module: crate::util::round::increment
// Provides: {"for_datetime"}
// Dependencies: {}
# [doc = " Validates the given rounding increment for the given unit."] # [doc = ""] # [doc = " This validation ensures the rounding increment is valid for rounding"] # [doc = " datetimes (both civil and time zone aware)."] pub (crate) fn for_datetime (unit : Unit , increment : i64 ,) -> Result < t :: NoUnits128 , Error > { static LIMIT : & [Constant] = & [t :: NANOS_PER_MICRO , t :: MICROS_PER_MILLI , t :: MILLIS_PER_SECOND , t :: SECONDS_PER_MINUTE , t :: MINUTES_PER_HOUR , t :: HOURS_PER_CIVIL_DAY , Constant (2) ,] ; get_with_limit (unit , increment , "datetime" , LIMIT) }
};
}
