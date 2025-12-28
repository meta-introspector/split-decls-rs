macro_rules! deps {
    () => {
        StashApplyOptions!();
    };
}

macro_rules! stash_apply_progress_cb {
    () => {
        deps!();
        extern "C" fn stash_apply_progress_cb (progress : raw :: git_stash_apply_progress_t , payload : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let options = & mut * (payload as * mut StashApplyOptions < '_ >) ; let res = { let callback = options . progress . as_mut () . unwrap () ; callback (convert_progress (progress)) } ; if res { 0 } else { - 1 } }) . unwrap_or (- 1) }
    };
}

stash_apply_progress_cb!();