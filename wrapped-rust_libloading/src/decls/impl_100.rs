macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < T > Clone for Symbol < T > { fn clone (& self) -> Symbol < T > { Symbol { .. * self } } }
    };
}

impl_100!()