macro_rules! deps {
    () => {
        RustcFieldIdx!();
        RustcEnumVariantIdx!();
    };
}

macro_rules! Layout {
    () => {
        deps!();
        pub type Layout = LayoutData < RustcFieldIdx , RustcEnumVariantIdx > ;
    };
}

Layout!();