macro_rules! deps {
    () => {
        CheckoutBuilder!();
    };
}

macro_rules! progress_cb {
    () => {
        deps!();
        extern "C" fn progress_cb (path : * const c_char , completed : size_t , total : size_t , data : * mut c_void ,) { panic :: wrap (| | unsafe { let payload = & mut * (data as * mut CheckoutBuilder < '_ >) ; let callback = match payload . progress { Some (ref mut c) => c , None => return , } ; let path = if path . is_null () { None } else { Some (util :: bytes2path (CStr :: from_ptr (path) . to_bytes ())) } ; callback (path , completed as usize , total as usize) }) ; }
    };
}

progress_cb!();