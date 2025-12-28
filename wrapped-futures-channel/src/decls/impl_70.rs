macro_rules! deps {
    () => {
        UnboundedSender!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T > Clone for UnboundedSender < T > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
    };
}

impl_70!()