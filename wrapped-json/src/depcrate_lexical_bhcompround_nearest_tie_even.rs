// Generated macro for round_nearest_tie_even (function)
macro_rules! Depcrate_lexical_bhcompround_nearest_tie_even {
() => {
// Module: crate::lexical::bhcomp
// Provides: {"round_nearest_tie_even"}
// Dependencies: {}
# [doc = " Custom round-nearest, tie-event algorithm for bhcomp."] # [inline] fn round_nearest_tie_even (fp : & mut ExtendedFloat , shift : i32 , is_truncated : bool) { let (mut is_above , mut is_halfway) = round_nearest (fp , shift) ; if is_halfway && is_truncated { is_above = true ; is_halfway = false ; } tie_even (fp , is_above , is_halfway) ; }
};
}
