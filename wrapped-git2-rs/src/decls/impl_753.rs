macro_rules! deps {
    () => {
        Statuses!();
    };
}

macro_rules! impl_753 {
    () => {
        deps!();
        impl < 'repo > Drop for Statuses < 'repo > { fn drop (& mut self) { unsafe { raw :: git_status_list_free (self . raw) ; } } }
    };
}

impl_753!();