// Generated macro for impl_75 (impl)
macro_rules! Depcrate_errorimpl_75 {
() => {
// Module: crate::error
// Provides: {"impl_75"}
// Dependencies: {}
impl Error { # [doc = " Creates an error object without any failure information."] pub const fn empty () -> Self { Self { code : S_EMPTY_ERROR , info : ErrorInfo :: empty () , } } # [doc = " Creates a new error object, capturing the stack and other information about the"] # [doc = " point of failure."] pub fn new < T : AsRef < str > > (code : HRESULT , message : T) -> Self { # [cfg (windows)] { let message : & str = message . as_ref () ; if message . is_empty () { Self :: from_hresult (code) } else { ErrorInfo :: originate_error (code , message) ; code . into () } } # [cfg (not (windows))] { let _ = message ; Self :: from_hresult (code) } } # [doc = " Creates a new error object with an error code, but without additional error information."] pub fn from_hresult (code : HRESULT) -> Self { Self { code : nonzero_hresult (code) , info : ErrorInfo :: empty () , } } # [doc = " Creates a new `Error` from the Win32 error code returned by `GetLastError()`."] pub fn from_thread () -> Self { Self :: from_hresult (HRESULT :: from_thread ()) } # [doc = " The error code describing the error."] pub const fn code (& self) -> HRESULT { if self . code . get () == S_EMPTY_ERROR . get () { HRESULT (0) } else { HRESULT (self . code . get ()) } } # [doc = " The error message describing the error."] pub fn message (& self) -> String { if let Some (message) = self . info . message () { return message ; } self . code () . message () } # [doc = " The error object describing the error."] # [cfg (windows)] pub fn as_ptr (& self) -> * mut core :: ffi :: c_void { self . info . as_ptr () } }
};
}
