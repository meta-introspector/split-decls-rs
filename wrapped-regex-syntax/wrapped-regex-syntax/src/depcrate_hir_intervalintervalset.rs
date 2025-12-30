// Generated macro for IntervalSet (struct)
macro_rules! Depcrate_hir_intervalIntervalSet {
() => {
// Module: crate::hir::interval
// Provides: {"IntervalSet"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct IntervalSet < I > { # [doc = " A sorted set of non-overlapping ranges."] ranges : Vec < I > , # [doc = " While not required at all for correctness, we keep track of whether an"] # [doc = " interval set has been case folded or not. This helps us avoid doing"] # [doc = " redundant work if, for example, a set has already been cased folded."] # [doc = " And note that whether a set is folded or not is preserved through"] # [doc = " all of the pairwise set operations. That is, if both interval sets"] # [doc = " have been case folded, then any of difference, union, intersection or"] # [doc = " symmetric difference all produce a case folded set."] # [doc = ""] # [doc = " Note that when this is true, it *must* be the case that the set is case"] # [doc = " folded. But when it's false, the set *may* be case folded. In other"] # [doc = " words, we only set this to true when we know it to be case, but we're"] # [doc = " okay with it being false if it would otherwise be costly to determine"] # [doc = " whether it should be true. This means code cannot assume that a false"] # [doc = " value necessarily indicates that the set is not case folded."] # [doc = ""] # [doc = " Bottom line: this is a performance optimization."] folded : bool , }
};
}
