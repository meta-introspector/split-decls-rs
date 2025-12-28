macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl windows_core :: TypeKind for Vector2 { type TypeKind = windows_core :: CopyType ; }
    };
}

impl_7!()