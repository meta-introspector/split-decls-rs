macro_rules! deps {
    () => {
        OutRef!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < 'a , T : Type < T > > From < & 'a mut T :: Default > for OutRef < 'a , T > { fn from (from : & 'a mut T :: Default) -> Self { unsafe { core :: mem :: transmute (from) } } }
    };
}

impl_172!()