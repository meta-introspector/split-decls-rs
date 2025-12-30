// Generated macro for is_week_designator (function)
macro_rules! Depcrate_parsers_grammaris_week_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_week_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `WeekDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_week_designator (ch : u8) -> bool { ch == b'W' || ch == b'w' }
};
}
