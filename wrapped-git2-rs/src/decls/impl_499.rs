macro_rules! deps {
    () => {
        OdbReader!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl < 'repo > Drop for OdbReader < 'repo > { fn drop (& mut self) { unsafe { raw :: git_odb_stream_free (self . raw) } } }
    };
}

impl_499!()