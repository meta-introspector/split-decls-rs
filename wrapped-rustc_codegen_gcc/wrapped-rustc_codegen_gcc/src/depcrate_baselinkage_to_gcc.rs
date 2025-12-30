// Generated macro for linkage_to_gcc (function)
macro_rules! Depcrate_baselinkage_to_gcc {
() => {
// Module: crate::base
// Provides: {"linkage_to_gcc"}
// Dependencies: {}
pub fn linkage_to_gcc (linkage : Linkage) -> FunctionType { match linkage { Linkage :: External => FunctionType :: Exported , Linkage :: AvailableExternally => FunctionType :: Extern , Linkage :: LinkOnceAny => unimplemented ! () , Linkage :: LinkOnceODR => unimplemented ! () , Linkage :: WeakAny => FunctionType :: Exported , Linkage :: WeakODR => unimplemented ! () , Linkage :: Internal => FunctionType :: Internal , Linkage :: ExternalWeak => unimplemented ! () , Linkage :: Common => unimplemented ! () , } }
};
}
