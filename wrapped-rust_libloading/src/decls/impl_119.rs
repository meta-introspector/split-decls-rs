macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T > Clone for Symbol < T > { fn clone (& self) -> Symbol < T > { Symbol { .. * self } } }
    };
}

impl_119!();