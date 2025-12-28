macro_rules! deps {
    () => {
        CheckoutBuilder!();
        DiffFile!();
    };
}

macro_rules! notify_cb {
    () => {
        deps!();
        extern "C" fn notify_cb (why : raw :: git_checkout_notify_t , path : * const c_char , baseline : * const raw :: git_diff_file , target : * const raw :: git_diff_file , workdir : * const raw :: git_diff_file , data : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let payload = & mut * (data as * mut CheckoutBuilder < '_ >) ; let callback = match payload . notify { Some (ref mut c) => c , None => return 0 , } ; let path = if path . is_null () { None } else { Some (util :: bytes2path (CStr :: from_ptr (path) . to_bytes ())) } ; let baseline = if baseline . is_null () { None } else { Some (DiffFile :: from_raw (baseline)) } ; let target = if target . is_null () { None } else { Some (DiffFile :: from_raw (target)) } ; let workdir = if workdir . is_null () { None } else { Some (DiffFile :: from_raw (workdir)) } ; let why = CheckoutNotificationType :: from_bits_truncate (why as u32) ; let keep_going = callback (why , path , baseline , target , workdir) ; if keep_going { 0 } else { 1 } }) . unwrap_or (2) }
    };
}

notify_cb!();