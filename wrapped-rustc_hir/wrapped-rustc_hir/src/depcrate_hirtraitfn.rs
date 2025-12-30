// Generated macro for TraitFn (enum)
macro_rules! Depcrate_hirTraitFn {
() => {
// Module: crate::hir
// Provides: {"TraitFn"}
// Dependencies: {}
# [doc = " Represents a trait method's body (or just argument names)."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum TraitFn < 'hir > { # [doc = " No default body in the trait, just a signature."] Required (& 'hir [Option < Ident >]) , # [doc = " Both signature and body are provided in the trait."] Provided (BodyId) , }
};
}
