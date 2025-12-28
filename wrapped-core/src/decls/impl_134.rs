macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl TypeKind for GUID { type TypeKind = CopyType ; }
    };
}

impl_134!()