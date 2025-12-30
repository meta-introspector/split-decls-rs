// Generated macro for EraYear (struct)
macro_rules! Depcrate_typesEraYear {
() => {
// Module: crate::types
// Provides: {"EraYear"}
// Dependencies: {}
# [doc = " Year information for a year that is specified with an era"] # [derive (Copy , Clone , Debug , PartialEq)] # [non_exhaustive] pub struct EraYear { # [doc = " The numeric year in that era"] pub year : i32 , # [doc = " See [`YearInfo::extended_year()`]"] pub extended_year : i32 , # [doc = " The era code as defined by CLDR, expect for cases where CLDR does not define a code."] pub era : TinyAsciiStr < 16 > , # [doc = " An era index, for calendars with a small set of eras."] # [doc = ""] # [doc = " The only guarantee we make is that these values are stable. These do *not*"] # [doc = " match the indices produced by ICU4C or CLDR."] # [doc = ""] # [doc = " These are used by ICU4X datetime formatting for efficiently storing data."] pub era_index : Option < u8 > , # [doc = " The ambiguity of the era/year combination"] pub ambiguity : YearAmbiguity , }
};
}
