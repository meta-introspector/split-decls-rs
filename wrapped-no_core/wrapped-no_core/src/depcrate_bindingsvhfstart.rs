// Generated macro for VhfStart (function)
macro_rules! Depcrate_bindingsVhfStart {
() => {
// Module: crate::bindings
// Provides: {"VhfStart"}
// Dependencies: {}
# [inline] pub unsafe fn VhfStart (vhfhandle : * const core :: ffi :: c_void) -> windows_result :: NTSTATUS { windows_link :: link ! ("vhfum.dll" "system" fn VhfStart (vhfhandle : * const core :: ffi :: c_void) -> windows_result :: NTSTATUS) ; unsafe { VhfStart (vhfhandle) } }
};
}
