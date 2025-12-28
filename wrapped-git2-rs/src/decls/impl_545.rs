macro_rules! deps {
    () => {
        Patch!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        unsafe impl < 'buffers > Send for Patch < 'buffers > { }
    };
}

impl_545!()