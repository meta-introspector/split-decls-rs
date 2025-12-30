// Generated macro for round_nearest_tie_even (function)
macro_rules! Depcrate_lexical_roundinground_nearest_tie_even {
() => {
// Module: crate::lexical::rounding
// Provides: {"round_nearest_tie_even"}
// Dependencies: {}
# [inline] pub (crate) fn round_nearest_tie_even (fp : & mut ExtendedFloat , shift : i32) { let (is_above , is_halfway) = round_nearest (fp , shift) ; tie_even (fp , is_above , is_halfway) ; }
};
}
