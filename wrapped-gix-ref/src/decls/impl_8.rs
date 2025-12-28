macro_rules! deps {
    () => {
        FullName!();
        FullNameRef!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a > From < & 'a FullNameRef > for FullName { fn from (value : & 'a FullNameRef) -> Self { FullName (value . as_bstr () . into ()) } }
    };
}

impl_8!()