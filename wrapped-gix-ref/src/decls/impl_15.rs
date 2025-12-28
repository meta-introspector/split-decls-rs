macro_rules! deps {
    () => {
        FullName!();
        FullNameRef!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Borrow < FullNameRef > for FullName { # [inline] fn borrow (& self) -> & FullNameRef { FullNameRef :: new_unchecked (self . 0 . as_bstr ()) } }
    };
}

impl_15!();