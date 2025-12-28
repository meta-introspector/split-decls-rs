macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl windows_core :: TypeKind for Vector4 { type TypeKind = windows_core :: CopyType ; }
    };
}

impl_13!();