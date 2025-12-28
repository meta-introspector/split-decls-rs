macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < T > Drop for Receiver < T > { fn drop (& mut self) { while ! self . object . is_empty () { self . recv () . unwrap () ; } } }
    };
}

impl_293!()