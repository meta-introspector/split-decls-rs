// Generated macro for for_timestamp (function)
macro_rules! Depcrate_util_round_incrementfor_timestamp {
() => {
// Module: crate::util::round::increment
// Provides: {"for_timestamp"}
// Dependencies: {}
# [doc = " Validates the given rounding increment for the given unit."] # [doc = ""] # [doc = " This validation ensures the rounding increment is valid for rounding"] # [doc = " timestamps."] pub (crate) fn for_timestamp (unit : Unit , increment : i64 ,) -> Result < t :: NoUnits128 , Error > { static MAX : & [Constant] = & [t :: NANOS_PER_CIVIL_DAY , t :: MICROS_PER_CIVIL_DAY , t :: MILLIS_PER_CIVIL_DAY , t :: SECONDS_PER_CIVIL_DAY , t :: MINUTES_PER_CIVIL_DAY , t :: HOURS_PER_CIVIL_DAY ,] ; get_with_max (unit , increment , "timestamp" , MAX) }
};
}
