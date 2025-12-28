macro_rules! deps {
    () => {
        Pathspec!();
    };
}

macro_rules! impl_558 {
    () => {
        deps!();
        impl Drop for Pathspec { fn drop (& mut self) { unsafe { raw :: git_pathspec_free (self . raw) } } }
    };
}

impl_558!()