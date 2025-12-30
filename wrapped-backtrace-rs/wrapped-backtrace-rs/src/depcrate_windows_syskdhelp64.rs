// Generated macro for KDHELP64 (struct)
macro_rules! Depcrate_windows_sysKDHELP64 {
() => {
// Module: crate::windows_sys
// Provides: {"KDHELP64"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct KDHELP64 { pub Thread : u64 , pub ThCallbackStack : u32 , pub ThCallbackBStore : u32 , pub NextCallback : u32 , pub FramePointer : u32 , pub KiCallUserMode : u64 , pub KeUserCallbackDispatcher : u64 , pub SystemRangeStart : u64 , pub KiUserExceptionDispatcher : u64 , pub StackBase : u64 , pub StackLimit : u64 , pub BuildVersion : u32 , pub RetpolineStubFunctionTableSize : u32 , pub RetpolineStubFunctionTable : u64 , pub RetpolineStubOffset : u32 , pub RetpolineStubSize : u32 , pub Reserved0 : [u64 ; 2] , }
};
}
