// Generated macro for UNWIND_HISTORY_TABLE (struct)
macro_rules! Depcrate_windows_sysUNWIND_HISTORY_TABLE {
() => {
// Module: crate::windows_sys
// Provides: {"UNWIND_HISTORY_TABLE"}
// Dependencies: {}
# [repr (C)] # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct UNWIND_HISTORY_TABLE { pub Count : u32 , pub LocalHint : u8 , pub GlobalHint : u8 , pub Search : u8 , pub Once : u8 , pub LowAddress : usize , pub HighAddress : usize , pub Entry : [UNWIND_HISTORY_TABLE_ENTRY ; 12] , }
};
}
