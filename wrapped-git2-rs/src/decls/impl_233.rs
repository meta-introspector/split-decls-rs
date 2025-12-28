macro_rules! deps {
    () => {
        Branches!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < 'repo > Drop for Branches < 'repo > { fn drop (& mut self) { unsafe { raw :: git_branch_iterator_free (self . raw) } } }
    };
}

impl_233!();