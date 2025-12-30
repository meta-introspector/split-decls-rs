// Generated macro for is_month_designator (function)
macro_rules! Depcrate_parsers_grammaris_month_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_month_designator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `MonthsDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_month_designator (ch : u8) -> bool { ch == b'M' || ch == b'm' }
};
}
