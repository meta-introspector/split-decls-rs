macro_rules! deps {
    () => {
        OnceLock!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T > Drop for OnceLock < T > { fn drop (& mut self) { if self . once . is_completed () { unsafe { self . value . get () . cast :: < T > () . drop_in_place () } ; } } }
    };
}

impl_92!()