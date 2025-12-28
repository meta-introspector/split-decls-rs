macro_rules! deps {
    () => {
        PushUpdate!();
        RemoteCallbacks!();
    };
}

macro_rules! push_negotiation_cb {
    () => {
        deps!();
        extern "C" fn push_negotiation_cb (updates : * mut * const raw :: git_push_update , len : size_t , payload : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let payload = & mut * (payload as * mut RemoteCallbacks < '_ >) ; let callback = match payload . push_negotiation { Some (ref mut c) => c , None => return 0 , } ; let updates = slice :: from_raw_parts (updates as * mut PushUpdate < '_ > , len) ; match callback (updates) { Ok (()) => 0 , Err (e) => e . raw_set_git_error () , } }) . unwrap_or (- 1) }
    };
}

push_negotiation_cb!();