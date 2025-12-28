macro_rules! deps {
    () => {
        Numeric!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T > Numeric for * mut T { fn into_u64 (self) -> u64 { self as u64 } fn from_u64 (src : u64) -> * mut T { src as * mut T } }
    };
}

impl_76!()