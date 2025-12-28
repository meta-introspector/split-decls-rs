macro_rules! deps {
    () => {
        Binding!();
        ApplyOptions!();
    };
}

macro_rules! delta_cb_c {
    () => {
        deps!();
        extern "C" fn delta_cb_c (delta : * const raw :: git_diff_delta , data : * mut c_void) -> c_int { panic :: wrap (| | unsafe { let delta = Binding :: from_raw_opt (delta as * mut _) ; let payload = & mut * (data as * mut ApplyOptions < '_ >) ; let callback = match payload . delta_cb { Some (ref mut c) => c , None => return - 1 , } ; let apply = callback (delta) ; if apply { 0 } else { 1 } }) . unwrap_or (- 1) }
    };
}

delta_cb_c!()