// Generated macro for is_second_designator (function)
macro_rules! Depcrate_parsers_grammaris_second_designator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_second_designator"}
// Dependencies: {}
# [doc = " checks if ascii char is a `SecondDesignator`."] # [inline] # [cfg (feature = "duration")] pub (crate) const fn is_second_designator (ch : u8) -> bool { ch == b'S' || ch == b's' }
};
}
