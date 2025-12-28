macro_rules! deps {
    () => {
        SourceName!();
        CtorDtorName!();
        UnnamedTypeName!();
        OperatorName!();
        ClosureTypeName!();
        UnqualifiedName!();
        IndexStr!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl UnqualifiedName { # [inline] fn starts_with (byte : u8 , input : & IndexStr) -> bool { byte == b'L' || OperatorName :: starts_with (byte) || CtorDtorName :: starts_with (byte) || SourceName :: starts_with (byte) || UnnamedTypeName :: starts_with (byte) || ClosureTypeName :: starts_with (byte , input) } }
    };
}

impl_116!();