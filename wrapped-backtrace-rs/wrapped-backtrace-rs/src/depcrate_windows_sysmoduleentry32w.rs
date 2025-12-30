// Generated macro for MODULEENTRY32W (struct)
macro_rules! Depcrate_windows_sysMODULEENTRY32W {
() => {
// Module: crate::windows_sys
// Provides: {"MODULEENTRY32W"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct MODULEENTRY32W { pub dwSize : u32 , pub th32ModuleID : u32 , pub th32ProcessID : u32 , pub GlblcntUsage : u32 , pub ProccntUsage : u32 , pub modBaseAddr : * mut u8 , pub modBaseSize : u32 , pub hModule : HMODULE , pub szModule : [u16 ; 256] , pub szExePath : [u16 ; 260] , }
};
}
