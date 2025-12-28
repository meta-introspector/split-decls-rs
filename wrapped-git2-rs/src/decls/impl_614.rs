macro_rules! deps {
    () => {
        References!();
    };
}

macro_rules! impl_614 {
    () => {
        deps!();
        impl < 'repo > Drop for References < 'repo > { fn drop (& mut self) { unsafe { raw :: git_reference_iterator_free (self . raw) } } }
    };
}

impl_614!()