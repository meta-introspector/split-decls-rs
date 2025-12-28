macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < 'repo > Drop for Blob < 'repo > { fn drop (& mut self) { unsafe { raw :: git_blob_free (self . raw) } } }
    };
}

impl_220!()