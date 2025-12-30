// Generated macro for ImplItemImplKind (enum)
macro_rules! Depcrate_hirImplItemImplKind {
() => {
// Module: crate::hir
// Provides: {"ImplItemImplKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum ImplItemImplKind { Inherent { vis_span : Span , } , Trait { defaultness : Defaultness , # [doc = " Item in the trait that this item implements"] trait_item_def_id : Result < DefId , ErrorGuaranteed > , } , }
};
}
