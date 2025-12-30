// Generated macro for CoInitializeEx (function)
macro_rules! Depcrate_fn_associated_enum_winCoInitializeEx {
() => {
// Module: crate::fn_associated_enum_win
// Provides: {"CoInitializeEx"}
// Dependencies: {}
# [inline] pub unsafe fn CoInitializeEx (pvreserved : Option < * const core :: ffi :: c_void > , dwcoinit : COINIT ,) -> windows_core :: HRESULT { windows_core :: link ! ("ole32.dll" "system" fn CoInitializeEx (pvreserved : * const core :: ffi :: c_void , dwcoinit : u32) -> windows_core :: HRESULT) ; unsafe { CoInitializeEx (pvreserved . unwrap_or (core :: mem :: zeroed ()) as _ , dwcoinit . 0 as _ ,) } }
};
}
