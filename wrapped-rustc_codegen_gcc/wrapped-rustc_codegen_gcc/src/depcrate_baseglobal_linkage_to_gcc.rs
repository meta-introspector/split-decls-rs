// Generated macro for global_linkage_to_gcc (function)
macro_rules! Depcrate_baseglobal_linkage_to_gcc {
() => {
// Module: crate::base
// Provides: {"global_linkage_to_gcc"}
// Dependencies: {}
pub fn global_linkage_to_gcc (linkage : Linkage) -> GlobalKind { match linkage { Linkage :: External => GlobalKind :: Imported , Linkage :: AvailableExternally => GlobalKind :: Imported , Linkage :: LinkOnceAny => unimplemented ! () , Linkage :: LinkOnceODR => unimplemented ! () , Linkage :: WeakAny => unimplemented ! () , Linkage :: WeakODR => unimplemented ! () , Linkage :: Internal => GlobalKind :: Internal , Linkage :: ExternalWeak => GlobalKind :: Imported , Linkage :: Common => unimplemented ! () , } }
};
}
