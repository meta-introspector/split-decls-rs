macro_rules! deps {
    () => {
        Odb!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        unsafe impl < 'repo > Sync for Odb < 'repo > { }
    };
}

impl_488!()