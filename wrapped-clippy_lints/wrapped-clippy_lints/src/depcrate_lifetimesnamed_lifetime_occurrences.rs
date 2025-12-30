// Generated macro for named_lifetime_occurrences (function)
macro_rules! Depcrate_lifetimesnamed_lifetime_occurrences {
() => {
// Module: crate::lifetimes
// Provides: {"named_lifetime_occurrences"}
// Dependencies: {}
# [doc = " Number of times each named lifetime occurs in the given slice. Returns a vector to preserve"] # [doc = " relative order."] # [must_use] fn named_lifetime_occurrences (lts : & [Lifetime]) -> Vec < (LocalDefId , usize) > { let mut occurrences = Vec :: new () ; for lt in lts { if let Some (curr_def_id) = named_lifetime (lt) { if let Some (pair) = occurrences . iter_mut () . find (| (prev_def_id , _) | * prev_def_id == curr_def_id) { pair . 1 += 1 ; } else { occurrences . push ((curr_def_id , 1)) ; } } } occurrences }
};
}
