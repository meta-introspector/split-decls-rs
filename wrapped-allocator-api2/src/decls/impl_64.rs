macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > AsRef < T > for Box < T , A > { # [inline (always)] fn as_ref (& self) -> & T { self } }
    };
}

impl_64!()