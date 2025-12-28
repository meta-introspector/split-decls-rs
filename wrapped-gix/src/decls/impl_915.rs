macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_915 {
    () => {
        deps!();
        impl AsRef < BStr > for Name < '_ > { fn as_ref (& self) -> & BStr { self . as_bstr () } }
    };
}

impl_915!()