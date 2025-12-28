macro_rules! deps {
    () => {
        Own!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        unsafe impl < T > Send for Own < T > where T : ? Sized { }
    };
}

impl_119!()