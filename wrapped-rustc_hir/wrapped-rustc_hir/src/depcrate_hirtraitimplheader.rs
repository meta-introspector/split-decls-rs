// Generated macro for TraitImplHeader (struct)
macro_rules! Depcrate_hirTraitImplHeader {
() => {
// Module: crate::hir
// Provides: {"TraitImplHeader"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct TraitImplHeader < 'hir > { pub constness : Constness , pub safety : Safety , pub polarity : ImplPolarity , pub defaultness : Defaultness , pub defaultness_span : Option < Span > , pub trait_ref : TraitRef < 'hir > , }
};
}
