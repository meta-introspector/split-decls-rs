macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T > Drop for AtomicCell < T > { fn drop (& mut self) { if mem :: needs_drop :: < T > () { unsafe { self . as_ptr () . drop_in_place () ; } } } }
    };
}

impl_19!()