macro_rules! deps {
    () => {
        Note!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl < 'repo > Drop for Note < 'repo > { fn drop (& mut self) { unsafe { raw :: git_note_free (self . raw) ; } } }
    };
}

impl_471!();