// Generated macro for VhfStart (function)
macro_rules! Depcrate_bindingsVhfStart {
() => {
// Module: crate::bindings
// Provides: {"VhfStart"}
// Dependencies: {}
# [inline] pub unsafe fn VhfStart (vhfhandle : * const core :: ffi :: c_void) -> windows_core :: NTSTATUS { windows_core :: link ! ("vhfum.dll" "system" fn VhfStart (vhfhandle : * const core :: ffi :: c_void) -> windows_core :: NTSTATUS) ; unsafe { VhfStart (vhfhandle) } }
};
}
