macro_rules! MAX_OBJECTS {
    () => {
        # [cfg (any (crossbeam_sanitize , miri))] const MAX_OBJECTS : usize = 4 ;
    };
}

MAX_OBJECTS!()