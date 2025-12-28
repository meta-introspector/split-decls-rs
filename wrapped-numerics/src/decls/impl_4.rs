macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl windows_core :: TypeKind for Matrix4x4 { type TypeKind = windows_core :: CopyType ; }
    };
}

impl_4!();