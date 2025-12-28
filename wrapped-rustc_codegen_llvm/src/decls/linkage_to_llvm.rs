macro_rules! deps {
    () => {
        Linkage!();
    };
}

macro_rules! linkage_to_llvm {
    () => {
        deps!();
        pub (crate) fn linkage_to_llvm (linkage : Linkage) -> llvm :: Linkage { match linkage { Linkage :: External => llvm :: Linkage :: ExternalLinkage , Linkage :: AvailableExternally => llvm :: Linkage :: AvailableExternallyLinkage , Linkage :: LinkOnceAny => llvm :: Linkage :: LinkOnceAnyLinkage , Linkage :: LinkOnceODR => llvm :: Linkage :: LinkOnceODRLinkage , Linkage :: WeakAny => llvm :: Linkage :: WeakAnyLinkage , Linkage :: WeakODR => llvm :: Linkage :: WeakODRLinkage , Linkage :: Internal => llvm :: Linkage :: InternalLinkage , Linkage :: ExternalWeak => llvm :: Linkage :: ExternalWeakLinkage , Linkage :: Common => llvm :: Linkage :: CommonLinkage , } }
    };
}

linkage_to_llvm!();