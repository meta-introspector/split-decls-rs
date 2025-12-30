// Generated macro for macro_1116 (macro)
macro_rules! Depcrate_ntldrmacro_1116 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1116"}
// Dependencies: {}
STRUCT ! { struct LDR_DATA_TABLE_ENTRY { InLoadOrderLinks : LIST_ENTRY , InMemoryOrderLinks : LIST_ENTRY , u1 : LDR_DATA_TABLE_ENTRY_u1 , DllBase : PVOID , EntryPoint : PLDR_INIT_ROUTINE , SizeOfImage : ULONG , FullDllName : UNICODE_STRING , BaseDllName : UNICODE_STRING , u2 : LDR_DATA_TABLE_ENTRY_u2 , ObsoleteLoadCount : USHORT , TlsIndex : USHORT , HashLinks : LIST_ENTRY , TimeDateStamp : ULONG , EntryPointActivationContext : * mut ACTIVATION_CONTEXT , Lock : PVOID , DdagNode : PLDR_DDAG_NODE , NodeModuleLink : LIST_ENTRY , LoadContext : * mut LDRP_LOAD_CONTEXT , ParentDllBase : PVOID , SwitchBackContext : PVOID , BaseAddressIndexNode : RTL_BALANCED_NODE , MappingInfoIndexNode : RTL_BALANCED_NODE , OriginalBase : ULONG_PTR , LoadTime : LARGE_INTEGER , BaseNameHashValue : ULONG , LoadReason : LDR_DLL_LOAD_REASON , ImplicitPathOptions : ULONG , ReferenceCount : ULONG , DependentLoadFlags : ULONG , SigningLevel : UCHAR , } }
};
}
