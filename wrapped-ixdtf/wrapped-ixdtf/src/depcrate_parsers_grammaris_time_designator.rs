// Generated macro for is_time_designator (function)
macro_rules! Depcrate_parsers_grammaris_time_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_time_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `TimeDesignator`."] # [inline] pub (crate) const fn is_time_designator (ch : u8) -> bool { ch == b'T' || ch == b't' }
};
}
