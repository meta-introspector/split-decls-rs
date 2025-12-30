// Generated macro for is_day_designator (function)
macro_rules! Depcrate_parsers_grammaris_day_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_day_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `DayDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_day_designator (ch : u8) -> bool { ch == b'D' || ch == b'd' }
};
}
