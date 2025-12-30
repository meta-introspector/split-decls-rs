// Generated macro for to_llvm_opt_settings (function)
macro_rules! Depcrate_back_writeto_llvm_opt_settings {
() => {
// Module: crate::back::write
// Provides: {"to_llvm_opt_settings"}
// Dependencies: {}
fn to_llvm_opt_settings (cfg : config :: OptLevel) -> (llvm :: CodeGenOptLevel , llvm :: CodeGenOptSize) { use self :: config :: OptLevel :: * ; match cfg { No => (llvm :: CodeGenOptLevel :: None , llvm :: CodeGenOptSizeNone) , Less => (llvm :: CodeGenOptLevel :: Less , llvm :: CodeGenOptSizeNone) , More => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeNone) , Aggressive => (llvm :: CodeGenOptLevel :: Aggressive , llvm :: CodeGenOptSizeNone) , Size => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeDefault) , SizeMin => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeAggressive) , } }
};
}
