macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < 'repo > Drop for Object < 'repo > { fn drop (& mut self) { unsafe { raw :: git_object_free (self . raw) } } }
    };
}

impl_484!();