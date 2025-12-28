macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < T , A : Allocator > AsRef < Vec < T , A > > for Vec < T , A > { # [inline (always)] fn as_ref (& self) -> & Vec < T , A > { self } }
    };
}

impl_178!();