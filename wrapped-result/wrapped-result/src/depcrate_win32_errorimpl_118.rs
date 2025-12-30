// Generated macro for impl_118 (impl)
macro_rules! Depcrate_win32_errorimpl_118 {
() => {
// Module: crate::win32_error
// Provides: {"impl_118"}
// Dependencies: {}
impl WIN32_ERROR { # [doc = " Returns [`true`] if `self` is a success code."] # [inline] pub const fn is_ok (self) -> bool { self . 0 == 0 } # [doc = " Returns [`true`] if `self` is a failure code."] # [inline] pub const fn is_err (self) -> bool { ! self . is_ok () } # [doc = " Maps a Win32 error code to an HRESULT value."] # [inline] pub const fn to_hresult (self) -> HRESULT { HRESULT (if self . 0 as i32 <= 0 { self . 0 } else { (self . 0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000 } as i32) } # [inline] pub fn from_error (error : & Error) -> Option < Self > { let hresult = error . code () . 0 as u32 ; if ((hresult >> 16) & 0x7FF) == 7 { Some (Self (hresult & 0xFFFF)) } else { None } } # [inline] pub fn ok (self) -> Result < () > { self . to_hresult () . ok () } # [doc = " Creates a new `WIN32_ERROR` from the Win32 error code returned by `GetLastError()`."] pub fn from_thread () -> Self { Self (unsafe { GetLastError () }) } }
};
}
