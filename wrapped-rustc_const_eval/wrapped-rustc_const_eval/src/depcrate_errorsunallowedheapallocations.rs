// Generated macro for UnallowedHeapAllocations (struct)
macro_rules! Depcrate_errorsUnallowedHeapAllocations {
() => {
// Module: crate::errors
// Provides: {"UnallowedHeapAllocations"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_unallowed_heap_allocations , code = E0010)] pub (crate) struct UnallowedHeapAllocations { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , # [note (const_eval_teach_note)] pub teach : bool , }
};
}
