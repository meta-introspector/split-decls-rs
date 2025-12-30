// Generated macro for pat_contains_disallowed_or (function)
macro_rules! Depcrate_matchespat_contains_disallowed_or {
() => {
// Module: crate::matches
// Provides: {"pat_contains_disallowed_or"}
// Dependencies: {}
# [doc = " Checks if `pat` contains OR patterns that cannot be nested due to a too low MSRV."] fn pat_contains_disallowed_or (cx : & LateContext < '_ > , pat : & Pat < '_ > , msrv : Msrv) -> bool { let mut contains_or = false ; pat . walk (| p | { let is_or = matches ! (p . kind , PatKind :: Or (_)) ; contains_or |= is_or ; ! is_or }) ; contains_or && ! msrv . meets (cx , msrvs :: OR_PATTERNS) }
};
}
