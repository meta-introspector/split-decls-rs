macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! revwalk_hide_cb {
    () => {
        deps!();
        extern "C" fn revwalk_hide_cb < C > (commit_id : * const raw :: git_oid , payload : * mut c_void) -> c_int where C : FnMut (Oid) -> bool , { panic :: wrap (| | unsafe { let hide_cb = payload as * mut C ; if (* hide_cb) (Oid :: from_raw (commit_id)) { 1 } else { 0 } }) . unwrap_or (- 1) }
    };
}

revwalk_hide_cb!();