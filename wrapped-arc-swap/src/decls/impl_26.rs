macro_rules! deps {
    () => {
        ConstantDeref!();
        Access!();
        Constant!();
        Guard!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T : Clone > Access < T > for Constant < T > { type Guard = ConstantDeref < T > ; fn load (& self) -> Self :: Guard { ConstantDeref (self . 0 . clone ()) } }
    };
}

impl_26!()