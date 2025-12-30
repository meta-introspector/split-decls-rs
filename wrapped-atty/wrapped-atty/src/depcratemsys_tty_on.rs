// Generated macro for msys_tty_on (function)
macro_rules! Depcratemsys_tty_on {
() => {
// Module: crate
// Provides: {"msys_tty_on"}
// Dependencies: {}
# [doc = " Returns true if there is an MSYS tty on the given handle."] # [cfg (windows)] unsafe fn msys_tty_on (fd : DWORD) -> bool { use std :: { mem , slice } ; use winapi :: { ctypes :: c_void , shared :: minwindef :: MAX_PATH , um :: { fileapi :: FILE_NAME_INFO , minwinbase :: FileNameInfo , processenv :: GetStdHandle , winbase :: GetFileInformationByHandleEx , } , } ; let size = mem :: size_of :: < FILE_NAME_INFO > () ; let mut name_info_bytes = vec ! [0u8 ; size + MAX_PATH * mem :: size_of ::< WCHAR > ()] ; let res = GetFileInformationByHandleEx (GetStdHandle (fd) , FileNameInfo , & mut * name_info_bytes as * mut _ as * mut c_void , name_info_bytes . len () as u32 ,) ; if res == 0 { return false ; } let name_info : & FILE_NAME_INFO = & * (name_info_bytes . as_ptr () as * const FILE_NAME_INFO) ; let s = slice :: from_raw_parts (name_info . FileName . as_ptr () , name_info . FileNameLength as usize / 2 ,) ; let name = String :: from_utf16_lossy (s) ; let is_msys = name . contains ("msys-") || name . contains ("cygwin-") ; let is_pty = name . contains ("-pty") ; is_msys && is_pty }
};
}
