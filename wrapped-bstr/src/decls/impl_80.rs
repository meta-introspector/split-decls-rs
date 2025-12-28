macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < 'a > ExactSizeIterator for Bytes < 'a > { # [inline] fn len (& self) -> usize { self . it . len () } }
    };
}

impl_80!()