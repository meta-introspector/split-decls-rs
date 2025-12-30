// Generated macro for write_offset (function)
macro_rules! Depcrate_fmt_strtime_formatwrite_offset {
() => {
// Module: crate::fmt::strtime::format
// Provides: {"write_offset"}
// Dependencies: {}
# [doc = " Writes the given time zone offset to the writer."] # [doc = ""] # [doc = " When `colon` is true, the hour, minute and optional second components are"] # [doc = " delimited by a colon. Otherwise, no delimiter is used."] # [doc = ""] # [doc = " When `minute` is true, the minute component is always printed. When"] # [doc = " false, the minute component is only printed when it is non-zero (or if"] # [doc = " the second component is non-zero)."] # [doc = ""] # [doc = " When `second` is true, the second component is always printed. When false,"] # [doc = " the second component is only printed when it is non-zero."] fn write_offset < W : Write > (offset : Offset , colon : bool , minute : bool , second : bool , wtr : & mut W ,) -> Result < () , Error > { static FMT_TWO : DecimalFormatter = DecimalFormatter :: new () . padding (2) ; let hours = offset . part_hours_ranged () . abs () . get () ; let minutes = offset . part_minutes_ranged () . abs () . get () ; let seconds = offset . part_seconds_ranged () . abs () . get () ; wtr . write_str (if offset . is_negative () { "-" } else { "+" }) ? ; wtr . write_int (& FMT_TWO , hours) ? ; if minute || minutes != 0 || seconds != 0 { if colon { wtr . write_str (":") ? ; } wtr . write_int (& FMT_TWO , minutes) ? ; if second || seconds != 0 { if colon { wtr . write_str (":") ? ; } wtr . write_int (& FMT_TWO , seconds) ? ; } } Ok (()) }
};
}
