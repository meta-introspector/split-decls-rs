macro_rules! deps {
    () => {
        OdbWriter!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl < 'repo > Drop for OdbWriter < 'repo > { fn drop (& mut self) { unsafe { raw :: git_odb_stream_free (self . raw) } } }
    };
}

impl_505!();