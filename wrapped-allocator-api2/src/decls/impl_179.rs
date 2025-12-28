macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < T , A : Allocator > AsMut < Vec < T , A > > for Vec < T , A > { # [inline (always)] fn as_mut (& mut self) -> & mut Vec < T , A > { self } }
    };
}

impl_179!();