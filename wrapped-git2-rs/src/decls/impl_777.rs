macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! impl_777 {
    () => {
        deps!();
        impl < 'repo > Drop for Tag < 'repo > { fn drop (& mut self) { unsafe { raw :: git_tag_free (self . raw) } } }
    };
}

impl_777!();