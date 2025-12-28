macro_rules! deps {
    () => {
        Element!();
        Padding!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl From < Padding > for Element < '_ > { fn from (value : Padding) -> Self { Self :: Padding (value) } }
    };
}

impl_125!();