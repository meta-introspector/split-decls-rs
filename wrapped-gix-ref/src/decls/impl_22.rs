macro_rules! deps {
    () => {
        PartialNameRef!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl PartialNameRef { pub (crate) fn new_unchecked (v : & BStr) -> & Self { # [allow (unsafe_code)] unsafe { std :: mem :: transmute (v) } } }
    };
}

impl_22!()