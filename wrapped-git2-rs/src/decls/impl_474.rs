macro_rules! deps {
    () => {
        Notes!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < 'repo > Drop for Notes < 'repo > { fn drop (& mut self) { unsafe { raw :: git_note_iterator_free (self . raw) ; } } }
    };
}

impl_474!();