macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T > Clone for Sender < T > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
    };
}

impl_69!()