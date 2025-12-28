macro_rules! local_size {
    () => {
        # [cfg (not (any (crossbeam_sanitize , miri)))] # [test] fn local_size () { }
    };
}

local_size!()