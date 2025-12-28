macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < I : ExactSizeIterator + ? Sized , A : Allocator > ExactSizeIterator for Box < I , A > { # [inline (always)] fn len (& self) -> usize { (* * self) . len () } }
    };
}

impl_58!()