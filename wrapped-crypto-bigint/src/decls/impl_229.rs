macro_rules! deps {
    () => {
        Limb!();
        Odd!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < T > AsRef < [Limb] > for Odd < T > where T : AsRef < [Limb] > , { fn as_ref (& self) -> & [Limb] { self . 0 . as_ref () } }
    };
}

impl_229!()