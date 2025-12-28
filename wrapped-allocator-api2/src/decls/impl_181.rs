macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < T , A : Allocator > AsMut < [T] > for Vec < T , A > { # [inline (always)] fn as_mut (& mut self) -> & mut [T] { self } }
    };
}

impl_181!();