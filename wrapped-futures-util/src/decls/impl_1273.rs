macro_rules! deps {
    () => {
        BiLockGuard!();
    };
}

macro_rules! impl_1273 {
    () => {
        deps!();
        impl < T > BiLockGuard < '_ , T > { # [doc = " Get a mutable pinned reference to the locked value."] pub fn as_pin_mut (& mut self) -> Pin < & mut T > { unsafe { Pin :: new_unchecked (& mut * self . bilock . arc . value . as_ref () . unwrap () . get ()) } } }
    };
}

impl_1273!()