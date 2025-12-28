macro_rules! deps {
    () => {
        FullNameRef!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl FullNameRef { pub (crate) fn new_unchecked (v : & BStr) -> & Self { # [allow (unsafe_code)] unsafe { std :: mem :: transmute (v) } } }
    };
}

impl_21!()