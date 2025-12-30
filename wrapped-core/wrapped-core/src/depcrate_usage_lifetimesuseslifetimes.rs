// Generated macro for UsesLifetimes (trait)
macro_rules! Depcrate_usage_lifetimesUsesLifetimes {
() => {
// Module: crate::usage::lifetimes
// Provides: {"UsesLifetimes"}
// Dependencies: {}
# [doc = " Searcher for finding lifetimes in a syntax tree."] # [doc = " This can be used to determine which lifetimes must be emitted in generated code."] pub trait UsesLifetimes { # [doc = " Returns the subset of the queried lifetimes that are used by the implementing syntax element."] # [doc = ""] # [doc = " This method only accounts for direct usage by the element; indirect usage via bounds or `where`"] # [doc = " predicates are not detected."] fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > ; # [doc = " Find all used lifetimes, then clone them and return that set."] fn uses_lifetimes_cloned (& self , options : & Options , lifetimes : & LifetimeSet) -> LifetimeSet { self . uses_lifetimes (options , lifetimes) . into_iter () . cloned () . collect () } }
};
}
