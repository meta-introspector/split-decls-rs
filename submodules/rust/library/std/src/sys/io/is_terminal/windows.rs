mkuse!{use crate :: ffi :: c_void ;}
mkuse!{use crate :: os :: windows :: io :: { AsHandle , AsRawHandle , BorrowedHandle } ;}
mkuse!{use crate :: sys :: c ;}

macro_rules! is_terminal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_terminal in module {}", module_path!());
    };
}

mkfn!{
    is_terminal_introspect!();
    pub fn is_terminal (h : & impl AsHandle) -> bool { handle_is_console (h . as_handle ()) }
}

macro_rules! handle_is_console_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_is_console in module {}", module_path!());
    };
}

mkfn!{
    handle_is_console_introspect!();
    fn handle_is_console (handle : BorrowedHandle < '_ >) -> bool { if handle . as_raw_handle () . is_null () { return false ; } let mut out = 0 ; if unsafe { c :: GetConsoleMode (handle . as_raw_handle () , & mut out) != 0 } { return true ; } msys_tty_on (handle) }
}

macro_rules! msys_tty_on_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function msys_tty_on in module {}", module_path!());
    };
}

mkfn!{
    msys_tty_on_introspect!();
    fn msys_tty_on (handle : BorrowedHandle < '_ >) -> bool { if unsafe { c :: GetFileType (handle . as_raw_handle ()) != c :: FILE_TYPE_PIPE } { return false ; } # [doc = " Mirrors [`FILE_NAME_INFO`], giving it a fixed length that we can stack"] # [doc = " allocate"] # [doc = ""] # [doc = " [`FILE_NAME_INFO`]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_name_info"] # [repr (C)] # [allow (non_snake_case)] struct FILE_NAME_INFO { FileNameLength : u32 , FileName : [u16 ; c :: MAX_PATH as usize] , } let mut name_info = FILE_NAME_INFO { FileNameLength : 0 , FileName : [0 ; c :: MAX_PATH as usize] } ; let res = unsafe { c :: GetFileInformationByHandleEx (handle . as_raw_handle () , c :: FileNameInfo , (& raw mut name_info) as * mut c_void , size_of :: < FILE_NAME_INFO > () as u32 ,) } ; if res == 0 { return false ; } let s = match name_info . FileName . get (.. name_info . FileNameLength as usize / 2) { None => return false , Some (s) => s , } ; let name = String :: from_utf16_lossy (s) ; let name = name . rsplit ('\\') . next () . unwrap_or (& name) ; let is_msys = name . starts_with ("msys-") || name . starts_with ("cygwin-") ; let is_pty = name . contains ("-pty") ; is_msys && is_pty }
}