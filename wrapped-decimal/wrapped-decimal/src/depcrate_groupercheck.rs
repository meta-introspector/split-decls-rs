// Generated macro for check (function)
macro_rules! Depcrate_groupercheck {
() => {
// Module: crate::grouper
// Provides: {"check"}
// Dependencies: {}
# [doc = " Returns whether to display a grouping separator at the given magnitude."] # [doc = ""] # [doc = " `upper_magnitude` is the magnitude of the highest-power digit, used for resolving minimum"] # [doc = " grouping digits."] pub fn check (upper_magnitude : i16 , magnitude : i16 , strategy : GroupingStrategy , sizes : GroupingSizes ,) -> bool { let primary = if sizes . primary == 0 { return false ; } else { sizes . primary as i16 } ; if magnitude < primary { return false ; } let min_grouping = { use GroupingStrategy :: * ; match strategy { Never => return false , Auto | Always => cmp :: max (1 , sizes . min_grouping) as i16 , Min2 => cmp :: max (2 , sizes . min_grouping) as i16 , } } ; if upper_magnitude < primary + min_grouping - 1 { return false ; } let secondary = if sizes . secondary == 0 { primary } else { sizes . secondary as i16 } ; let magnitude_prime = magnitude - primary ; if magnitude_prime % secondary == 0 { return true ; } false }
};
}
