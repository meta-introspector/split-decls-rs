macro_rules! deps {
    () => {
        BuiltinType!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl From < BuiltinType > for hir_def :: builtin_type :: BuiltinType { fn from (it : BuiltinType) -> Self { it . inner } }
    };
}

impl_44!();