macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl Drop for Index { fn drop (& mut self) { unsafe { raw :: git_index_free (self . raw) } } }
    };
}

impl_391!()