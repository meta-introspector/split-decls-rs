macro_rules! deps {
    () => {
        Owned!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T : Clone > Clone for Owned < T > { fn clone (& self) -> Self { Self :: new ((* * self) . clone ()) . with_tag (self . tag ()) } }
    };
}

impl_40!();