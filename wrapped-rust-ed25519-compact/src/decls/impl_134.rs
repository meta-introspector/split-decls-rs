macro_rules! deps {
    () => {
        PublicKey!();
        DHOutput!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl From < DHOutput > for PublicKey { fn from (dh : DHOutput) -> Self { PublicKey (dh . 0) } }
    };
}

impl_134!()