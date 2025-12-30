// Generated macro for GenericBound (enum)
macro_rules! Depcrate_hirGenericBound {
() => {
// Module: crate::hir
// Provides: {"GenericBound"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , HashStable_Generic)] pub enum GenericBound < 'hir > { Trait (PolyTraitRef < 'hir >) , Outlives (& 'hir Lifetime) , Use (& 'hir [PreciseCapturingArg < 'hir >] , Span) , }
};
}
