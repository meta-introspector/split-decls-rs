// Generated macro for CollectLifetimes (trait)
macro_rules! Depcrate_usage_lifetimesCollectLifetimes {
() => {
// Module: crate::usage::lifetimes
// Provides: {"CollectLifetimes"}
// Dependencies: {}
# [doc = " Searcher for finding lifetimes in an iterator."] # [doc = ""] # [doc = " This trait extends iterators, providing a way to turn a filtered list of fields or variants into a set"] # [doc = " of lifetimes."] pub trait CollectLifetimes { # [doc = " Consume an iterator, accumulating all lifetimes in the elements which occur in `lifetimes`."] fn collect_lifetimes < 'a > (self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > ; # [doc = " Consume an iterator using `collect_lifetimes`, then clone all found lifetimes and return that set."] fn collect_lifetimes_cloned (self , options : & Options , lifetimes : & LifetimeSet) -> LifetimeSet ; }
};
}
