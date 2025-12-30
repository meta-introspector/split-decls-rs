// Generated macro for is_hour_designator (function)
macro_rules! Depcrate_parsers_grammaris_hour_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_hour_designator"}
// Dependencies: {}
# [doc = " checks if ascii char is a `DayDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_hour_designator (ch : u8) -> bool { ch == b'H' || ch == b'h' }
};
}
