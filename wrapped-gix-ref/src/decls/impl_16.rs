macro_rules! deps {
    () => {
        FullNameRef!();
        FullName!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl AsRef < FullNameRef > for FullName { fn as_ref (& self) -> & FullNameRef { self . borrow () } }
    };
}

impl_16!();