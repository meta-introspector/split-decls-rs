// Generated macro for recurse_or_patterns (function)
macro_rules! Depcraterecurse_or_patterns {
() => {
// Module: crate
// Provides: {"recurse_or_patterns"}
// Dependencies: {}
# [doc = " If the pattern is an `or` pattern, call the function once for each sub pattern. Otherwise, call"] # [doc = " the function once on the given pattern."] pub fn recurse_or_patterns < 'tcx , F : FnMut (& 'tcx Pat < 'tcx >) > (pat : & 'tcx Pat < 'tcx > , mut f : F) { if let PatKind :: Or (pats) = pat . kind { pats . iter () . for_each (f) ; } else { f (pat) ; } }
};
}
