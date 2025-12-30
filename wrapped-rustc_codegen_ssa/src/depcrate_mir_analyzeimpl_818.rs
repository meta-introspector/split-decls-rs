// Generated macro for impl_818 (impl)
macro_rules! Depcrate_mir_analyzeimpl_818 {
() => {
// Module: crate::mir::analyze
// Provides: {"impl_818"}
// Dependencies: {}
impl CleanupKind { pub (crate) fn funclet_bb (self , for_bb : mir :: BasicBlock) -> Option < mir :: BasicBlock > { match self { CleanupKind :: NotCleanup => None , CleanupKind :: Funclet => Some (for_bb) , CleanupKind :: Internal { funclet } => Some (funclet) , } } }
};
}
