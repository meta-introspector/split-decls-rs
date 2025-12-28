macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > DerefMut for Box < T , A > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { unsafe { self . 0 . as_mut () } } }
    };
}

impl_53!();