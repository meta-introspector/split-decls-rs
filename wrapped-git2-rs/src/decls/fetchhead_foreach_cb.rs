macro_rules! deps {
    () => {
        FetchheadForeachCbData!();
        Binding!();
    };
}

macro_rules! fetchhead_foreach_cb {
    () => {
        deps!();
        extern "C" fn fetchhead_foreach_cb (ref_name : * const c_char , remote_url : * const c_char , oid : * const raw :: git_oid , is_merge : c_uint , payload : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let data = & mut * (payload as * mut FetchheadForeachCbData < '_ >) ; let res = { let callback = & mut data . callback ; assert ! (! ref_name . is_null ()) ; assert ! (! remote_url . is_null ()) ; assert ! (! oid . is_null ()) ; let ref_name = str :: from_utf8 (CStr :: from_ptr (ref_name) . to_bytes ()) . unwrap () ; let remote_url = CStr :: from_ptr (remote_url) . to_bytes () ; let oid = Binding :: from_raw (oid) ; let is_merge = is_merge == 1 ; callback (& ref_name , remote_url , & oid , is_merge) } ; if res { 0 } else { 1 } }) . unwrap_or (1) }
    };
}

fetchhead_foreach_cb!();