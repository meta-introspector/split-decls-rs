macro_rules! deps {
    () => {
        FoldStop!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T , E > From < E > for FoldStop < T , E > { # [inline] fn from (e : E) -> FoldStop < T , E > { FoldStop :: Err (e) } }
    };
}

impl_3!()