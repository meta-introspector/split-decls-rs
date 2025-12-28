macro_rules! deps {
    () => {
        RemoteCallbacks!();
    };
}

macro_rules! push_transfer_progress_cb {
    () => {
        deps!();
        extern "C" fn push_transfer_progress_cb (progress : c_uint , total : c_uint , bytes : size_t , data : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let payload = & mut * (data as * mut RemoteCallbacks < '_ >) ; let callback = match payload . push_progress { Some (ref mut c) => c , None => return 0 , } ; callback (progress as usize , total as usize , bytes as usize) ; 0 }) . unwrap_or (- 1) }
    };
}

push_transfer_progress_cb!()