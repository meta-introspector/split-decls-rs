macro_rules! deps {
    () => {
        DiffCallbacks!();
        Binding!();
    };
}

macro_rules! file_cb_c {
    () => {
        deps!();
        pub extern "C" fn file_cb_c (delta : * const raw :: git_diff_delta , progress : f32 , data : * mut c_void ,) -> c_int { unsafe { let delta = Binding :: from_raw (delta as * mut _) ; let r = panic :: wrap (| | { let cbs = data as * mut DiffCallbacks < '_ , '_ , '_ , '_ , '_ , '_ , '_ , '_ > ; match (* cbs) . file { Some (ref mut cb) => cb (delta , progress) , None => false , } }) ; if r == Some (true) { raw :: GIT_OK } else { raw :: GIT_EUSER } } }
    };
}

file_cb_c!()