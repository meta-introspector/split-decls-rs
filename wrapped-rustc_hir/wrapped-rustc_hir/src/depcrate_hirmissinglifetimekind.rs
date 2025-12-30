// Generated macro for MissingLifetimeKind (enum)
macro_rules! Depcrate_hirMissingLifetimeKind {
() => {
// Module: crate::hir
// Provides: {"MissingLifetimeKind"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , HashStable_Generic , Debug)] pub enum MissingLifetimeKind { # [doc = " An explicit `'_`."] Underscore , # [doc = " An elided lifetime `&' ty`."] Ampersand , # [doc = " An elided lifetime in brackets with written brackets."] Comma , # [doc = " An elided lifetime with elided brackets."] Brackets , }
};
}
