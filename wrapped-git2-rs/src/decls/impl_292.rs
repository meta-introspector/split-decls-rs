macro_rules! deps {
    () => {
        Describe!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < 'repo > Drop for Describe < 'repo > { fn drop (& mut self) { unsafe { raw :: git_describe_result_free (self . raw) } } }
    };
}

impl_292!();