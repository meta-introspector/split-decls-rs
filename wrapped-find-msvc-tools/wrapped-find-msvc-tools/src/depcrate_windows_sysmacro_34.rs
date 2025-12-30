// Generated macro for macro_34 (macro)
macro_rules! Depcrate_windows_sysmacro_34 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_34"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn GetMachineTypeAttributes (machine : u16 , machinetypeattributes : * mut MACHINE_ATTRIBUTES) -> HRESULT) ;
};
}
