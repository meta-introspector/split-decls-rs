// Generated macro for VALID_RD_RANGE (const)
macro_rules! Depcrate_calendar_arithmeticVALID_RD_RANGE {
() => {
// Module: crate::calendar_arithmetic
// Provides: {"VALID_RD_RANGE"}
// Dependencies: {}
# [doc = " This is a fundamental invariant of `ArithmeticDate` and by extension all our"] # [doc = " date types. Because this range slightly exceeds the [`VALID_YEAR_RANGE`], only"] # [doc = " the valid year range is checked in constructors. Only the `Date::from_rata_die`"] # [doc = " constructor actually uses this, but for clamping instead of erroring."] pub const VALID_RD_RANGE : RangeInclusive < RataDie > = RataDie :: new (- 367256444) ..= RataDie :: new (365940477) ;
};
}
