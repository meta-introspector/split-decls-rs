macro_rules! deps {
    () => {
        AtomicPtr!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < T > From < * mut T > for AtomicPtr < T > { fn from (p : * mut T) -> Self { Self :: new (p) } }
    };
}

impl_273!();