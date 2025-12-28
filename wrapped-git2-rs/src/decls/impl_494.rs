macro_rules! deps {
    () => {
        OdbObject!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl < 'a > Drop for OdbObject < 'a > { fn drop (& mut self) { unsafe { raw :: git_odb_object_free (self . raw) } } }
    };
}

impl_494!();