// Generated macro for is_utc_designator (function)
macro_rules! Depcrate_parsers_grammaris_utc_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_utc_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `UtcDesignator`."] # [inline] pub (crate) const fn is_utc_designator (ch : u8) -> bool { ch == b'Z' || ch == b'z' }
};
}
