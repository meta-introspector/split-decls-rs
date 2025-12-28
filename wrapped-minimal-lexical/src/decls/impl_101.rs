macro_rules! deps {
    () => {
        FastPathRadix!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl From < FastPathRadix > for u64 { fn from (radix : FastPathRadix) -> u64 { match radix { FastPathRadix :: Five => 5 , FastPathRadix :: Ten => 10 , } } }
    };
}

impl_101!();