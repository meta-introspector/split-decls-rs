// Generated macro for to_pass_builder_opt_level (function)
macro_rules! Depcrate_back_writeto_pass_builder_opt_level {
() => {
// Module: crate::back::write
// Provides: {"to_pass_builder_opt_level"}
// Dependencies: {}
fn to_pass_builder_opt_level (cfg : config :: OptLevel) -> llvm :: PassBuilderOptLevel { use config :: OptLevel :: * ; match cfg { No => llvm :: PassBuilderOptLevel :: O0 , Less => llvm :: PassBuilderOptLevel :: O1 , More => llvm :: PassBuilderOptLevel :: O2 , Aggressive => llvm :: PassBuilderOptLevel :: O3 , Size => llvm :: PassBuilderOptLevel :: Os , SizeMin => llvm :: PassBuilderOptLevel :: Oz , } }
};
}
