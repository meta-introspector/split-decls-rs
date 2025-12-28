macro_rules! deps {
    () => {
        CowMut!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < T : Default > Default for CowMut < '_ , T > { fn default () -> Self { CowMut :: Owned (T :: default ()) } }
    };
}

impl_118!();