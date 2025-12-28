macro_rules! deps {
    () => {
        Odb!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        unsafe impl < 'repo > Send for Odb < 'repo > { }
    };
}

impl_487!()