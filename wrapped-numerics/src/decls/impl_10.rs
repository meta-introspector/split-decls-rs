macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl windows_core :: TypeKind for Vector3 { type TypeKind = windows_core :: CopyType ; }
    };
}

impl_10!();