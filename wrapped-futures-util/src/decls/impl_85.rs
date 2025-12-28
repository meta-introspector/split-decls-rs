macro_rules! deps {
    () => {
        WeakShared!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < Fut : Future > Clone for WeakShared < Fut > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
    };
}

impl_85!();