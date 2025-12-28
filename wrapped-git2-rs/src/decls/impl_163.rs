macro_rules! deps {
    () => {
        StringArray!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Drop for StringArray { fn drop (& mut self) { unsafe { raw :: git_strarray_free (& mut self . raw) } } }
    };
}

impl_163!();