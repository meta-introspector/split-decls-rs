macro_rules! deps {
    () => {
        RustcEnumVariantIdx!();
    };
}

macro_rules! TagEncoding {
    () => {
        deps!();
        pub type TagEncoding = hir_def :: layout :: TagEncoding < RustcEnumVariantIdx > ;
    };
}

TagEncoding!();