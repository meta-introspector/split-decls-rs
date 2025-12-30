// Generated macro for TyPatKind (enum)
macro_rules! Depcrate_hirTyPatKind {
() => {
// Module: crate::hir
// Provides: {"TyPatKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum TyPatKind < 'hir > { # [doc = " A range pattern (e.g., `1..=2` or `1..2`)."] Range (& 'hir ConstArg < 'hir > , & 'hir ConstArg < 'hir >) , # [doc = " A list of patterns where only one needs to be satisfied"] Or (& 'hir [TyPat < 'hir >]) , # [doc = " A placeholder for a pattern that wasn't well formed in some way."] Err (ErrorGuaranteed) , }
};
}
