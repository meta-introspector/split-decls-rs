macro_rules! deps {
    () => {
        RustcEnumVariantIdx!();
    };
}

macro_rules! impl_650 {
    () => {
        deps!();
        impl rustc_index :: Idx for RustcEnumVariantIdx { fn new (idx : usize) -> Self { RustcEnumVariantIdx (idx) } fn index (self) -> usize { self . 0 } }
    };
}

impl_650!();