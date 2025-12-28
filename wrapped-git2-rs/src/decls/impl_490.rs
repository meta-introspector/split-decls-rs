macro_rules! deps {
    () => {
        Odb!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        impl < 'repo > Drop for Odb < 'repo > { fn drop (& mut self) { unsafe { raw :: git_odb_free (self . raw) } } }
    };
}

impl_490!();