// Generated macro for macro_1274 (macro)
macro_rules! Depcrate_ntlpcapimacro_1274 {
() => {
// Module: crate::ntlpcapi
// Provides: {"macro_1274"}
// Dependencies: {}
STRUCT ! { # [repr (align (128))] struct ALPC_COMPLETION_LIST_HEADER { StartMagic : ULONG64 , TotalSize : ULONG , ListOffset : ULONG , ListSize : ULONG , BitmapOffset : ULONG , BitmapSize : ULONG , DataOffset : ULONG , DataSize : ULONG , AttributeFlags : ULONG , AttributeSize : ULONG , __padding0 : [u64 ; 10] , State : ALPC_COMPLETION_LIST_STATE , LastMessageId : ULONG , LastCallbackId : ULONG , __padding1 : [u32 ; 28] , PostCount : ULONG , __padding2 : [u32 ; 31] , ReturnCount : ULONG , __padding3 : [u32 ; 31] , LogSequenceNumber : ULONG , __padding4 : [u64 ; 15] , UserLock : RTL_SRWLOCK , EndMagic : ULONG64 , __padding5 : [u64 ; 14] , } }
};
}
