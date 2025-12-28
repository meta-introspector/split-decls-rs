macro_rules! deps {
    () => {
        MergeheadForeachCbData!();
        Binding!();
    };
}

macro_rules! mergehead_foreach_cb {
    () => {
        deps!();
        extern "C" fn mergehead_foreach_cb (oid : * const raw :: git_oid , payload : * mut c_void) -> c_int { panic :: wrap (| | unsafe { let data = & mut * (payload as * mut MergeheadForeachCbData < '_ >) ; let res = { let callback = & mut data . callback ; callback (& Binding :: from_raw (oid)) } ; if res { 0 } else { 1 } }) . unwrap_or (1) }
    };
}

mergehead_foreach_cb!();