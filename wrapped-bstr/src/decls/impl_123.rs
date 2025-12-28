macro_rules! deps {
    () => {
        DrainBytes!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'a > ExactSizeIterator for DrainBytes < 'a > { # [inline] fn len (& self) -> usize { self . it . len () } }
    };
}

impl_123!();