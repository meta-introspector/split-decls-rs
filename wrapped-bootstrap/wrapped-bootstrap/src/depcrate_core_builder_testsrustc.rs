// Generated macro for rustc (macro)
macro_rules! Depcrate_core_builder_testsrustc {
() => {
// Module: crate::core::builder::tests
// Provides: {"rustc"}
// Dependencies: {}
macro_rules ! rustc { ($ host : ident => $ target : ident , stage = $ stage : literal) => { compile :: Rustc :: new (Compiler :: new ($ stage , TargetSelection :: from_user ($ host)) , TargetSelection :: from_user ($ target) ,) } ; }
};
}
