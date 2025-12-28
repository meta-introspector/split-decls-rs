macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Deref for Owned < T > { type Target = T ; fn deref (& self) -> & T { let (raw , _) = decompose_tag :: < T > (self . data) ; unsafe { T :: deref (raw) } } }
    };
}

impl_41!();