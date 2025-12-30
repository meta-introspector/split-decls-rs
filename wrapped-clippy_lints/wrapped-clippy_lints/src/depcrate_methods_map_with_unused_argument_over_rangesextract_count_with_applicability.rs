// Generated macro for extract_count_with_applicability (function)
macro_rules! Depcrate_methods_map_with_unused_argument_over_rangesextract_count_with_applicability {
() => {
// Module: crate::methods::map_with_unused_argument_over_ranges
// Provides: {"extract_count_with_applicability"}
// Dependencies: {}
fn extract_count_with_applicability (cx : & LateContext < '_ > , range : higher :: Range < '_ > , applicability : & mut Applicability ,) -> Option < String > { let start = range . start ? ; let end = range . end ? ; if let ExprKind :: Lit (lit) = start . kind && let LitKind :: Int (Pu128 (lower_bound) , _) = lit . node { if let ExprKind :: Lit (lit) = end . kind && let LitKind :: Int (Pu128 (upper_bound) , _) = lit . node { let count = if upper_bound >= lower_bound { match range . limits { RangeLimits :: HalfOpen => upper_bound - lower_bound , RangeLimits :: Closed => (upper_bound - lower_bound) . checked_add (1) ? , } } else { 0 } ; return Some (format ! ("{count}")) ; } let end_snippet = Sugg :: hir_with_applicability (cx , end , "..." , applicability) . maybe_paren () . into_string () ; if lower_bound == 0 { if range . limits == RangeLimits :: Closed { return Some (format ! ("{end_snippet} + 1")) ; } return Some (end_snippet) ; } if range . limits == RangeLimits :: Closed { return Some (format ! ("{end_snippet} - {}" , lower_bound - 1)) ; } return Some (format ! ("{end_snippet} - {lower_bound}")) ; } None }
};
}
