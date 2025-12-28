macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > DerefMut for Owned < T > { fn deref_mut (& mut self) -> & mut T { let (raw , _) = decompose_tag :: < T > (self . data) ; unsafe { T :: deref_mut (raw) } } }
    };
}

impl_42!()