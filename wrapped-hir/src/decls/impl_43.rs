macro_rules! deps {
    () => {
        BuiltinType!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl From < hir_def :: builtin_type :: BuiltinType > for BuiltinType { fn from (inner : hir_def :: builtin_type :: BuiltinType) -> Self { Self { inner } } }
    };
}

impl_43!();