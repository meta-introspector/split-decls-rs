// Generated macro for std (macro)
macro_rules! Depcrate_core_builder_testsstd {
() => {
// Module: crate::core::builder::tests
// Provides: {"std"}
// Dependencies: {}
macro_rules ! std { ($ host : ident => $ target : ident , stage = $ stage : literal) => { compile :: Std :: new (Compiler :: new ($ stage , TargetSelection :: from_user ($ host)) , TargetSelection :: from_user ($ target) ,) } ; }
};
}
