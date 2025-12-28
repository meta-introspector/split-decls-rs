macro_rules! deps {
    () => {
        PrintCb!();
        Binding!();
    };
}

macro_rules! print_cb {
    () => {
        deps!();
        pub extern "C" fn print_cb (delta : * const raw :: git_diff_delta , hunk : * const raw :: git_diff_hunk , line : * const raw :: git_diff_line , data : * mut c_void ,) -> c_int { unsafe { let delta = Binding :: from_raw (delta as * mut _) ; let hunk = Binding :: from_raw_opt (hunk) ; let line = Binding :: from_raw (line) ; let r = panic :: wrap (| | { let data = data as * mut & mut PrintCb < '_ > ; (* data) (delta , hunk , line) }) ; if r == Some (true) { raw :: GIT_OK } else { raw :: GIT_EUSER } } }
    };
}

print_cb!();