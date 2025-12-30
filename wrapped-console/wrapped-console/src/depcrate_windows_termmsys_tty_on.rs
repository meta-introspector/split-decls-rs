// Generated macro for msys_tty_on (function)
macro_rules! Depcrate_windows_termmsys_tty_on {
() => {
// Module: crate::windows_term
// Provides: {"msys_tty_on"}
// Dependencies: {}
# [doc = " Returns true if there is an MSYS tty on the given handle."] pub (crate) fn msys_tty_on (term : & Term) -> bool { let handle = term . as_raw_handle () ; unsafe { { let mut out = MaybeUninit :: uninit () ; let res = GetConsoleMode (handle as HANDLE , out . as_mut_ptr ()) ; if res != 0 && (out . assume_init () & ENABLE_VIRTUAL_TERMINAL_PROCESSING) == ENABLE_VIRTUAL_TERMINAL_PROCESSING { return true ; } } # [doc = " Mirrors windows_sys::Win32::Storage::FileSystem::FILE_NAME_INFO, giving"] # [doc = " it a fixed length that we can stack allocate"] # [repr (C)] # [allow (non_snake_case)] struct FILE_NAME_INFO { FileNameLength : u32 , FileName : [u16 ; MAX_PATH as usize] , } let mut name_info = FILE_NAME_INFO { FileNameLength : 0 , FileName : [0 ; MAX_PATH as usize] , } ; let res = GetFileInformationByHandleEx (handle as HANDLE , FileNameInfo , & mut name_info as * mut _ as * mut c_void , mem :: size_of :: < FILE_NAME_INFO > () as u32 ,) ; if res == 0 { return false ; } let s = match name_info . FileName . get (.. name_info . FileNameLength as usize / 2) { Some (s) => s , None => return false , } ; let name = String :: from_utf16_lossy (s) ; let is_msys = name . contains ("msys-") || name . contains ("cygwin-") ; let is_pty = name . contains ("-pty") ; is_msys && is_pty } }
};
}
