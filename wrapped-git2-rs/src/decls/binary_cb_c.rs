macro_rules! deps {
    () => {
        Binding!();
        DiffCallbacks!();
    };
}

macro_rules! binary_cb_c {
    () => {
        deps!();
        pub extern "C" fn binary_cb_c (delta : * const raw :: git_diff_delta , binary : * const raw :: git_diff_binary , data : * mut c_void ,) -> c_int { unsafe { let delta = Binding :: from_raw (delta as * mut _) ; let binary = Binding :: from_raw (binary) ; let r = panic :: wrap (| | { let cbs = data as * mut DiffCallbacks < '_ , '_ , '_ , '_ , '_ , '_ , '_ , '_ > ; match (* cbs) . binary { Some (ref mut cb) => cb (delta , binary) , None => false , } }) ; if r == Some (true) { raw :: GIT_OK } else { raw :: GIT_EUSER } } }
    };
}

binary_cb_c!()