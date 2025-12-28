macro_rules! deps {
    () => {
        Patch!();
    };
}

macro_rules! impl_547 {
    () => {
        deps!();
        impl < 'buffers > Drop for Patch < 'buffers > { fn drop (& mut self) { unsafe { raw :: git_patch_free (self . raw) } } }
    };
}

impl_547!();