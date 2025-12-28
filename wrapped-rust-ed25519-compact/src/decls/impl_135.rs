macro_rules! deps {
    () => {
        DHOutput!();
        SecretKey!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl From < DHOutput > for SecretKey { fn from (dh : DHOutput) -> Self { SecretKey (dh . 0) } }
    };
}

impl_135!();