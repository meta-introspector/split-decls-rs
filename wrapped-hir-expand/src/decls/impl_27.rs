macro_rules! deps {
    () => {
        RenderedExpandError!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl RenderedExpandError { const GENERAL_KIND : & str = "macro-error" ; const DISABLED : & str = "proc-macro-disabled" ; const ATTR_EXP_DISABLED : & str = "attribute-expansion-disabled" ; }
    };
}

impl_27!()