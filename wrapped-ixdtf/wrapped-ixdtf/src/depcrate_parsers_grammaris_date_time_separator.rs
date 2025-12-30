// Generated macro for is_date_time_separator (function)
macro_rules! Depcrate_parsers_grammaris_date_time_separator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_date_time_separator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `DateTimeSeparator`."] # [inline] pub (crate) const fn is_date_time_separator (ch : u8) -> bool { is_time_designator (ch) || is_space (ch) }
};
}
