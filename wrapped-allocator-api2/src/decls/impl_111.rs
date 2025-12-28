macro_rules! deps {
    () => {
        Drain!();
        Allocator!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < 'a , T , A : Allocator > AsRef < [T] > for Drain < 'a , T , A > { # [inline (always)] fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_111!()