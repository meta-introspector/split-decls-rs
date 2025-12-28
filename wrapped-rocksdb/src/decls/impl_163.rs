macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        unsafe impl Send for Options { }
    };
}

impl_163!()