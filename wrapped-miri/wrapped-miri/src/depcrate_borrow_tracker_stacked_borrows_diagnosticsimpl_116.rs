// Generated macro for impl_116 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsimpl_116 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'ecx , 'tcx > DiagnosticCxBuilder < 'ecx , 'tcx > { pub fn build < 'history > (self , history : & 'history mut AllocHistory , offset : Size ,) -> DiagnosticCx < 'history , 'ecx , 'tcx > { DiagnosticCx { operation : self . operation , machine : self . machine , history , offset } } pub fn retag (machine : & 'ecx MiriMachine < 'tcx > , info : RetagInfo , new_tag : BorTag , orig_tag : ProvenanceExtra , range : AllocRange ,) -> Self { let operation = Operation :: Retag (RetagOp { info , new_tag , orig_tag , range , permission : None }) ; DiagnosticCxBuilder { machine , operation } } pub fn read (machine : & 'ecx MiriMachine < 'tcx > , tag : ProvenanceExtra , range : AllocRange) -> Self { let operation = Operation :: Access (AccessOp { kind : AccessKind :: Read , tag , range }) ; DiagnosticCxBuilder { machine , operation } } pub fn write (machine : & 'ecx MiriMachine < 'tcx > , tag : ProvenanceExtra , range : AllocRange ,) -> Self { let operation = Operation :: Access (AccessOp { kind : AccessKind :: Write , tag , range }) ; DiagnosticCxBuilder { machine , operation } } pub fn dealloc (machine : & 'ecx MiriMachine < 'tcx > , tag : ProvenanceExtra) -> Self { let operation = Operation :: Dealloc (DeallocOp { tag }) ; DiagnosticCxBuilder { machine , operation } } }
};
}
