// Generated macro for offset (function)
macro_rules! Depcrate_tzoffset {
() => {
// Module: crate::tz
// Provides: {"offset"}
// Dependencies: {}
# [doc = " Creates a new time zone offset in a `const` context from a given number"] # [doc = " of hours."] # [doc = ""] # [doc = " Negative offsets correspond to time zones west of the prime meridian,"] # [doc = " while positive offsets correspond to time zones east of the prime"] # [doc = " meridian. Equivalently, in all cases, `civil-time - offset = UTC`."] # [doc = ""] # [doc = " The fallible non-const version of this constructor is"] # [doc = " [`Offset::from_hours`]."] # [doc = ""] # [doc = " This is a convenience free function for [`Offset::constant`]. It is"] # [doc = " intended to provide a terse syntax for constructing `Offset` values from"] # [doc = " a value that is known to be valid."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This routine panics when the given number of hours is out of range."] # [doc = " Namely, `hours` must be in the range `-25..=25`."] # [doc = ""] # [doc = " Similarly, when used in a const context, an out of bounds hour will prevent"] # [doc = " your Rust program from compiling."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::tz::offset;"] # [doc = ""] # [doc = " let o = offset(-5);"] # [doc = " assert_eq!(o.seconds(), -18_000);"] # [doc = " let o = offset(5);"] # [doc = " assert_eq!(o.seconds(), 18_000);"] # [doc = " ```"] # [inline] pub const fn offset (hours : i8) -> Offset { Offset :: constant (hours) }
};
}
