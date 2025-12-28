macro_rules! deps {
    () => {
        RustcEnumVariantIdx!();
        RustcFieldIdx!();
    };
}

macro_rules! Variants {
    () => {
        deps!();
        pub type Variants = hir_def :: layout :: Variants < RustcFieldIdx , RustcEnumVariantIdx > ;
    };
}

Variants!()