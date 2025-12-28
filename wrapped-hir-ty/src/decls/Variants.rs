macro_rules! deps {
    () => {
        RustcFieldIdx!();
        RustcEnumVariantIdx!();
    };
}

macro_rules! Variants {
    () => {
        deps!();
        pub type Variants = hir_def :: layout :: Variants < RustcFieldIdx , RustcEnumVariantIdx > ;
    };
}

Variants!();