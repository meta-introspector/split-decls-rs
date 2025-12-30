// Generated macro for UnificationTable (type)
macro_rules! Depcrate_inferUnificationTable {
() => {
// Module: crate::infer
// Provides: {"UnificationTable"}
// Dependencies: {}
pub (crate) type UnificationTable < 'a , 'tcx , T > = ut :: UnificationTable < ut :: InPlace < T , & 'a mut ut :: UnificationStorage < T > , & 'a mut InferCtxtUndoLogs < 'tcx > > , > ;
};
}
