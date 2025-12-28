macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > AsMut < T > for Box < T , A > { # [inline (always)] fn as_mut (& mut self) -> & mut T { self } }
    };
}

impl_65!();