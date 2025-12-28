macro_rules! deps {
    () => {
        TypeDef!();
        NestedClass!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < 'a > NestedClass < 'a > { pub fn inner (& self) -> TypeDef < 'a > { self . row (0) } pub fn outer (& self) -> TypeDef < 'a > { self . row (1) } }
    };
}

impl_115!();