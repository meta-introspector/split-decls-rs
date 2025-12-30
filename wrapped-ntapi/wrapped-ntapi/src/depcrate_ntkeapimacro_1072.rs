// Generated macro for macro_1072 (macro)
macro_rules! Depcrate_ntkeapimacro_1072 {
() => {
// Module: crate::ntkeapi
// Provides: {"macro_1072"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCallbackReturn (OutputBuffer : PVOID , OutputLength : ULONG , Status : NTSTATUS ,) -> NTSTATUS ; fn NtFlushProcessWriteBuffers () ; fn NtQueryDebugFilterState (ComponentId : ULONG , Level : ULONG ,) -> NTSTATUS ; fn NtSetDebugFilterState (ComponentId : ULONG , Level : ULONG , State : BOOLEAN ,) -> NTSTATUS ; fn NtYieldExecution () -> NTSTATUS ; } }
};
}
