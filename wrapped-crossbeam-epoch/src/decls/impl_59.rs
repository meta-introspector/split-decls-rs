macro_rules! deps {
    () => {
        Pointable!();
        Shared!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'g , T : ? Sized + Pointable > PartialOrd < Shared < 'g , T > > for Shared < 'g , T > { fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { Some (self . data . cmp (& other . data)) } }
    };
}

impl_59!()