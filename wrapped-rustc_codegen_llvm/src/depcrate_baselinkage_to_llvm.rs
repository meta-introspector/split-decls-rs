// Generated macro for linkage_to_llvm (function)
macro_rules! Depcrate_baselinkage_to_llvm {
() => {
// Module: crate::base
// Provides: {"linkage_to_llvm"}
// Dependencies: {}
pub (crate) fn linkage_to_llvm (linkage : Linkage) -> llvm :: Linkage { match linkage { Linkage :: External => llvm :: Linkage :: ExternalLinkage , Linkage :: AvailableExternally => llvm :: Linkage :: AvailableExternallyLinkage , Linkage :: LinkOnceAny => llvm :: Linkage :: LinkOnceAnyLinkage , Linkage :: LinkOnceODR => llvm :: Linkage :: LinkOnceODRLinkage , Linkage :: WeakAny => llvm :: Linkage :: WeakAnyLinkage , Linkage :: WeakODR => llvm :: Linkage :: WeakODRLinkage , Linkage :: Internal => llvm :: Linkage :: InternalLinkage , Linkage :: ExternalWeak => llvm :: Linkage :: ExternalWeakLinkage , Linkage :: Common => llvm :: Linkage :: CommonLinkage , } }
};
}
