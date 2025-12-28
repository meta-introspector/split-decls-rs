macro_rules! deps {
    () => {
        FilterCb!();
    };
}

macro_rules! filter_cb {
    () => {
        deps!();
        extern "C" fn filter_cb (entry : * const raw :: git_tree_entry , payload : * mut c_void) -> c_int { let ret = panic :: wrap (| | unsafe { if panic :: panicked () { true } else { let entry = tree :: entry_from_raw_const (entry) ; let payload = payload as * mut & mut FilterCb < '_ > ; (* payload) (& entry) } }) ; if ret == Some (false) { 1 } else { 0 } }
    };
}

filter_cb!();