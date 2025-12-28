macro_rules! deps {
    () => {
        DiffStats!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl Drop for DiffStats { fn drop (& mut self) { unsafe { raw :: git_diff_stats_free (self . raw) } } }
    };
}

impl_352!()