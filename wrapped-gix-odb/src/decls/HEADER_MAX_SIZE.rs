macro_rules! HEADER_MAX_SIZE {
    () => {
        # [doc = " The maximum size that an object header can have. `git2` says 64, and `git` says 32 but also mentions it can be larger."] const HEADER_MAX_SIZE : usize = 64 ;
    };
}

HEADER_MAX_SIZE!();