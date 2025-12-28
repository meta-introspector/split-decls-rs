macro_rules! deps {
    () => {
        Pointable!();
        Shared!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'g , T : ? Sized + Pointable > PartialEq < Shared < 'g , T > > for Shared < 'g , T > { fn eq (& self , other : & Self) -> bool { self . data == other . data } }
    };
}

impl_57!();