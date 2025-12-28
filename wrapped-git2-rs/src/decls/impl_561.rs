macro_rules! deps {
    () => {
        PathspecMatchList!();
    };
}

macro_rules! impl_561 {
    () => {
        deps!();
        impl < 'ps > Drop for PathspecMatchList < 'ps > { fn drop (& mut self) { unsafe { raw :: git_pathspec_match_list_free (self . raw) } } }
    };
}

impl_561!();