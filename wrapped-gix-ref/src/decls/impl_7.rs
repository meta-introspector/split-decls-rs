macro_rules! deps {
    () => {
        FullNameRef!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > From < & 'a FullNameRef > for & 'a BStr { fn from (name : & 'a FullNameRef) -> Self { & name . 0 } }
    };
}

impl_7!();