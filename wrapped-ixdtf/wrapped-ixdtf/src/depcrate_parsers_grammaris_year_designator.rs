// Generated macro for is_year_designator (function)
macro_rules! Depcrate_parsers_grammaris_year_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_year_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `YearDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_year_designator (ch : u8) -> bool { ch == b'Y' || ch == b'y' }
};
}
