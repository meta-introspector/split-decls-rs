macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > borrow :: Borrow < T > for Box < T , A > { # [inline (always)] fn borrow (& self) -> & T { self } }
    };
}

impl_62!();