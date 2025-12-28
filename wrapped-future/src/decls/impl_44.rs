macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl windows_core :: TypeKind for AsyncStatus { type TypeKind = windows_core :: CopyType ; }
    };
}

impl_44!()