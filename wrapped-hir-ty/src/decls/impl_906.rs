macro_rules! deps {
    () => {
        MirSpan!();
    };
}

macro_rules! impl_906 {
    () => {
        deps!();
        impl From < & ExprId > for MirSpan { fn from (value : & ExprId) -> Self { (* value) . into () } }
    };
}

impl_906!()