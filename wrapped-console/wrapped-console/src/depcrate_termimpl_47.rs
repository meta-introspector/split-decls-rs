// Generated macro for impl_47 (impl)
macro_rules! Depcrate_termimpl_47 {
() => {
// Module: crate::term
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (windows)] impl AsRawHandle for Term { fn as_raw_handle (& self) -> RawHandle { use windows_sys :: Win32 :: System :: Console :: { GetStdHandle , STD_ERROR_HANDLE , STD_OUTPUT_HANDLE , } ; unsafe { GetStdHandle (match self . inner . target { TermTarget :: Stdout => STD_OUTPUT_HANDLE , TermTarget :: Stderr => STD_ERROR_HANDLE , }) as RawHandle } } }
};
}
