macro_rules! deps {
    () => {
        RemoteCallbacks!();
    };
}

macro_rules! credentials_cb {
    () => {
        deps!();
        extern "C" fn credentials_cb (ret : * mut * mut raw :: git_cred , url : * const c_char , username_from_url : * const c_char , allowed_types : c_uint , payload : * mut c_void ,) -> c_int { unsafe { let ok = panic :: wrap (| | { let payload = & mut * (payload as * mut RemoteCallbacks < '_ >) ; let callback = payload . credentials . as_mut () . ok_or (raw :: GIT_PASSTHROUGH as c_int) ? ; * ret = ptr :: null_mut () ; let url = str :: from_utf8 (CStr :: from_ptr (url) . to_bytes ()) . map_err (| _ | raw :: GIT_PASSTHROUGH as c_int) ? ; let username_from_url = match crate :: opt_bytes (& url , username_from_url) { Some (username) => { Some (str :: from_utf8 (username) . map_err (| _ | raw :: GIT_PASSTHROUGH as c_int) ?) } None => None , } ; let cred_type = CredentialType :: from_bits_truncate (allowed_types as u32) ; callback (url , username_from_url , cred_type) . map_err (| e | e . raw_set_git_error ()) }) ; match ok { Some (Ok (cred)) => { if allowed_types & (cred . credtype () as c_uint) != 0 { * ret = cred . unwrap () ; 0 } else { raw :: GIT_PASSTHROUGH as c_int } } Some (Err (e)) => e , None => - 1 , } } }
    };
}

credentials_cb!()