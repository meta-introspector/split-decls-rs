// Generated macro for is_duration_designator (function)
macro_rules! Depcrate_parsers_grammaris_duration_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_duration_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `DurationDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_duration_designator (ch : u8) -> bool { ch == b'P' || ch == b'p' }
};
}
