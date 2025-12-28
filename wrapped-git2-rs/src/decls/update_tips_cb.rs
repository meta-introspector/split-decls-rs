macro_rules! deps {
    () => {
        Binding!();
        RemoteCallbacks!();
    };
}

macro_rules! update_tips_cb {
    () => {
        deps!();
        extern "C" fn update_tips_cb (refname : * const c_char , a : * const raw :: git_oid , b : * const raw :: git_oid , data : * mut c_void ,) -> c_int { let ok = panic :: wrap (| | unsafe { let payload = & mut * (data as * mut RemoteCallbacks < '_ >) ; let callback = match payload . update_tips { Some (ref mut c) => c , None => return true , } ; let refname = str :: from_utf8 (CStr :: from_ptr (refname) . to_bytes ()) . unwrap () ; let a = Binding :: from_raw (a) ; let b = Binding :: from_raw (b) ; callback (refname , a , b) }) ; if ok == Some (true) { 0 } else { - 1 } }
    };
}

update_tips_cb!();