// Generated macro for is_minute_designator (function)
macro_rules! Depcrate_parsers_grammaris_minute_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_minute_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `MinuteDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_minute_designator (ch : u8) -> bool { is_month_designator (ch) }
};
}
