// Generated macro for visibility_to_llvm (function)
macro_rules! Depcrate_basevisibility_to_llvm {
() => {
// Module: crate::base
// Provides: {"visibility_to_llvm"}
// Dependencies: {}
pub (crate) fn visibility_to_llvm (linkage : Visibility) -> llvm :: Visibility { match linkage { Visibility :: Default => llvm :: Visibility :: Default , Visibility :: Hidden => llvm :: Visibility :: Hidden , Visibility :: Protected => llvm :: Visibility :: Protected , } }
};
}
