macro_rules! deps {
    () => {
        RustcEnumVariantIdx!();
        RustcFieldIdx!();
    };
}

macro_rules! Layout {
    () => {
        deps!();
        pub type Layout = LayoutData < RustcFieldIdx , RustcEnumVariantIdx > ;
    };
}

Layout!()