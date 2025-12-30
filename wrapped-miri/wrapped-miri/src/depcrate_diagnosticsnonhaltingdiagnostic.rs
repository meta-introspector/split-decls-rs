// Generated macro for NonHaltingDiagnostic (enum)
macro_rules! Depcrate_diagnosticsNonHaltingDiagnostic {
() => {
// Module: crate::diagnostics
// Provides: {"NonHaltingDiagnostic"}
// Dependencies: {}
# [doc = " Miri specific diagnostics"] pub enum NonHaltingDiagnostic { # [doc = " (new_tag, new_perm, (alloc_id, base_offset, orig_tag))"] # [doc = ""] # [doc = " new_perm is `None` for base tags."] CreatedPointerTag (NonZero < u64 > , Option < String > , Option < (AllocId , AllocRange , ProvenanceExtra) >) , # [doc = " This `Item` was popped from the borrow stack. The string explains the reason."] PoppedPointerTag (Item , String) , CreatedAlloc (AllocId , Size , Align , MemoryKind) , FreedAlloc (AllocId) , AccessedAlloc (AllocId , AccessKind) , RejectedIsolatedOp (String) , ProgressReport { block_count : u64 , } , Int2Ptr { details : bool , } , NativeCallSharedMem { tracing : bool , } , WeakMemoryOutdatedLoad { ptr : Pointer , } , ExternTypeReborrow , }
};
}
