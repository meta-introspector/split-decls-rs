macro_rules! deps {
    () => {
        FARPROC!();
        BOOL!();
        DWORD!();
        HMODULE!();
        HANDLE!();
    };
}

macro_rules! windows_imports {
    () => {
        deps!();
        # [cfg (any (not (libloading_docs) , windows))] mod windows_imports { use super :: { BOOL , DWORD , FARPROC , HANDLE , HMODULE } ; windows_link :: link ! ("kernel32.dll" "system" fn GetLastError () -> DWORD) ; windows_link :: link ! ("kernel32.dll" "system" fn SetThreadErrorMode (new_mode : DWORD , old_mode : * mut DWORD) -> BOOL) ; windows_link :: link ! ("kernel32.dll" "system" fn GetModuleHandleExW (flags : u32 , module_name : * const u16 , module : * mut HMODULE) -> BOOL) ; windows_link :: link ! ("kernel32.dll" "system" fn FreeLibrary (module : HMODULE) -> BOOL) ; windows_link :: link ! ("kernel32.dll" "system" fn LoadLibraryExW (filename : * const u16 , file : HANDLE , flags : DWORD) -> HMODULE) ; windows_link :: link ! ("kernel32.dll" "system" fn GetModuleFileNameW (module : HMODULE , filename : * mut u16 , size : DWORD) -> DWORD) ; windows_link :: link ! ("kernel32.dll" "system" fn GetProcAddress (module : HMODULE , procname : * const u8) -> FARPROC) ; }
    };
}

windows_imports!();