macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > Deref for Box < T , A > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { unsafe { self . 0 . as_ref () } } }
    };
}

impl_52!()