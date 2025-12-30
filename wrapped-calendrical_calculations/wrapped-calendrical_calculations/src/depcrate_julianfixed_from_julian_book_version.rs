// Generated macro for fixed_from_julian_book_version (function)
macro_rules! Depcrate_julianfixed_from_julian_book_version {
() => {
// Module: crate::julian
// Provides: {"fixed_from_julian_book_version"}
// Dependencies: {}
# [doc = " Get a fixed date from the ymd of a Julian date."] # [doc = ""] # [doc = " Years are counted as in _Calendrical Calculations_ by Reingold & Dershowitz,"] # [doc = " meaning there is no year 0. For instance, near the epoch date, years are counted: -3, -2, -1, 1, 2, 3 instead of -2, -1, 0, 1, 2, 3."] # [doc = ""] # [doc = " Primarily useful for use with code constructing epochs specified in the bookg"] pub const fn fixed_from_julian_book_version (book_year : i32 , month : u8 , day : u8) -> RataDie { debug_assert ! (book_year != 0) ; fixed_from_julian (if book_year < 0 { book_year + 1 } else { book_year } , month , day ,) }
};
}
