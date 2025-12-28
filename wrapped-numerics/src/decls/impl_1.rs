macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl windows_core :: TypeKind for Matrix3x2 { type TypeKind = windows_core :: CopyType ; }
    };
}

impl_1!()