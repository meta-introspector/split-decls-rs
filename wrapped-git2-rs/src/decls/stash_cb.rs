macro_rules! deps {
    () => {
        StashCbData!();
        Binding!();
    };
}

macro_rules! stash_cb {
    () => {
        deps!();
        pub (crate) extern "C" fn stash_cb (index : size_t , message : * const c_char , stash_id : * const raw :: git_oid , payload : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let data = & mut * (payload as * mut StashCbData < '_ >) ; let res = { let callback = & mut data . callback ; callback (index , CStr :: from_ptr (message) . to_str () . unwrap () , & Binding :: from_raw (stash_id) ,) } ; if res { 0 } else { 1 } }) . unwrap_or (1) }
    };
}

stash_cb!();