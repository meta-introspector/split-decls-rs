macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < T , A : Allocator > AsRef < [T] > for Vec < T , A > { # [inline (always)] fn as_ref (& self) -> & [T] { self } }
    };
}

impl_180!()