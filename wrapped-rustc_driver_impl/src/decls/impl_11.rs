macro_rules! deps {
    () => {
        AstNoAnn!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl pprust_ast :: PpAnn for AstNoAnn { }
    };
}

impl_11!()