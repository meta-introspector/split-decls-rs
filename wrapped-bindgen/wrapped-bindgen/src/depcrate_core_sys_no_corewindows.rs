// Generated macro for Windows (module)
macro_rules! Depcrate_core_sys_no_coreWindows {
() => {
// Module: crate::core_sys_no_core
// Provides: {"Windows"}
// Dependencies: {}
pub mod Windows { pub mod Win32 { pub mod System { pub mod Com { windows_link :: link ! ("ole32.dll" "system" fn CoCreateGuid (pguid : * mut super :: super :: super :: super :: GUID) -> super :: super :: super :: super :: HRESULT) ; } } } }
};
}
