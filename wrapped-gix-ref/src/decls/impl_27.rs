macro_rules! deps {
    () => {
        FullNameRef!();
        PartialNameRef!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a > From < & 'a FullNameRef > for & 'a PartialNameRef { fn from (v : & 'a FullNameRef) -> Self { PartialNameRef :: new_unchecked (v . 0 . as_bstr ()) } }
    };
}

impl_27!();