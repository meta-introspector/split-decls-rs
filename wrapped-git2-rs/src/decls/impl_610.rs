macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl < 'repo > Drop for Reference < 'repo > { fn drop (& mut self) { unsafe { raw :: git_reference_free (self . raw) } } }
    };
}

impl_610!();