// Generated macro for ElidableUsage (enum)
macro_rules! Depcrate_lifetimesElidableUsage {
() => {
// Module: crate::lifetimes
// Provides: {"ElidableUsage"}
// Dependencies: {}
# [derive (Copy , Clone)] enum ElidableUsage { # [doc = " Used in a ref (`&'a T`), can be removed"] Ref (Span) , # [doc = " Used as a generic param (`T<'a>`) or an impl lifetime (`impl T + 'a`), can be replaced"] # [doc = " with `'_`"] Other (Span) , }
};
}
