// Generated macro for doc_std (macro)
macro_rules! Depcrate_core_builder_testsdoc_std {
() => {
// Module: crate::core::builder::tests
// Provides: {"doc_std"}
// Dependencies: {}
macro_rules ! doc_std { ($ host : ident => $ target : ident , stage = $ stage : literal) => { { doc :: Std :: new ($ stage , TargetSelection :: from_user ($ target) , DocumentationFormat :: Html) } } ; }
};
}
