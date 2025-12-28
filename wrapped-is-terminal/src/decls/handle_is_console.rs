macro_rules! handle_is_console {
    () => {
        # [cfg (windows)] fn handle_is_console (handle : BorrowedHandle < '_ >) -> bool { use windows_sys :: Win32 :: System :: Console :: GetConsoleMode ; let handle = handle . as_raw_handle () ; if handle . is_null () { return false ; } unsafe { let mut out = 0 ; if GetConsoleMode (handle as HANDLE , & mut out) != 0 { return true ; } msys_tty_on (handle as HANDLE) } }
    };
}

handle_is_console!()