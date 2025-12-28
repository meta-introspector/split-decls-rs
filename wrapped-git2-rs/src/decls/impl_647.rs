macro_rules! deps {
    () => {
        Remote!();
    };
}

macro_rules! impl_647 {
    () => {
        deps!();
        impl < 'repo > Drop for Remote < 'repo > { fn drop (& mut self) { unsafe { raw :: git_remote_free (self . raw) } } }
    };
}

impl_647!()