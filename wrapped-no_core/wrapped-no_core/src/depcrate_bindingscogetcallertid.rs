// Generated macro for CoGetCallerTID (function)
macro_rules! Depcrate_bindingsCoGetCallerTID {
() => {
// Module: crate::bindings
// Provides: {"CoGetCallerTID"}
// Dependencies: {}
# [inline] pub unsafe fn CoGetCallerTID () -> windows_result :: Result < u32 > { windows_link :: link ! ("ole32.dll" "system" fn CoGetCallerTID (lpdwtid : * mut u32) -> windows_result :: HRESULT) ; unsafe { let mut result__ = core :: mem :: zeroed () ; CoGetCallerTID (& mut result__) . map (| | result__) } }
};
}
