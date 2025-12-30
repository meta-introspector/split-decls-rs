// Generated macro for macro_46 (macro)
macro_rules! Depcrate_ntdbgmacro_46 {
() => {
// Module: crate::ntdbg
// Provides: {"macro_46"}
// Dependencies: {}
EXTERN ! { extern "system" { fn vDbgPrintEx (ComponentId : ULONG , Level : ULONG , Format : PCCH , arglist : va_list ,) -> ULONG ; fn vDbgPrintExWithPrefix (Prefix : PCH , ComponentId : ULONG , Level : ULONG , Format : PCCH , arglist : va_list ,) -> ULONG ; fn DbgQueryDebugFilterState (ComponentId : ULONG , Level : ULONG ,) -> NTSTATUS ; fn DbgSetDebugFilterState (ComponentId : ULONG , Level : ULONG , State : BOOLEAN ,) -> NTSTATUS ; fn DbgPrompt (Prompt : PCCH , Response : PCH , Length : ULONG ,) -> ULONG ; } }
};
}
