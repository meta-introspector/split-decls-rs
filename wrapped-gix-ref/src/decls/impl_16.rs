macro_rules! deps {
    () => {
        FullName!();
        FullNameRef!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl AsRef < FullNameRef > for FullName { fn as_ref (& self) -> & FullNameRef { self . borrow () } }
    };
}

impl_16!()