// Generated macro for for_span (function)
macro_rules! Depcrate_util_round_incrementfor_span {
() => {
// Module: crate::util::round::increment
// Provides: {"for_span"}
// Dependencies: {}
# [doc = " Validates the given rounding increment for the given unit."] # [doc = ""] # [doc = " This validation ensures the rounding increment is valid for rounding spans."] pub (crate) fn for_span (unit : Unit , increment : i64 ,) -> Result < t :: NoUnits128 , Error > { static LIMIT : & [Constant] = & [t :: NANOS_PER_MICRO , t :: MICROS_PER_MILLI , t :: MILLIS_PER_SECOND , t :: SECONDS_PER_MINUTE , t :: MINUTES_PER_HOUR , t :: HOURS_PER_CIVIL_DAY ,] ; if unit >= Unit :: Day { Ok (t :: NoUnits128 :: rfrom (t :: NoUnits :: new_unchecked (increment))) } else { get_with_limit (unit , increment , "span" , LIMIT) } }
};
}
