macro_rules! deps {
    () => {
        Binding!();
        DiffCallbacks!();
    };
}

macro_rules! hunk_cb_c {
    () => {
        deps!();
        pub extern "C" fn hunk_cb_c (delta : * const raw :: git_diff_delta , hunk : * const raw :: git_diff_hunk , data : * mut c_void ,) -> c_int { unsafe { let delta = Binding :: from_raw (delta as * mut _) ; let hunk = Binding :: from_raw (hunk) ; let r = panic :: wrap (| | { let cbs = data as * mut DiffCallbacks < '_ , '_ , '_ , '_ , '_ , '_ , '_ , '_ > ; match (* cbs) . hunk { Some (ref mut cb) => cb (delta , hunk) , None => false , } }) ; if r == Some (true) { raw :: GIT_OK } else { raw :: GIT_EUSER } } }
    };
}

hunk_cb_c!()