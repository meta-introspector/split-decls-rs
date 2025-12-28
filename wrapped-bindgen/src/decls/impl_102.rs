macro_rules! impl_102 {
    () => {
        impl NestedClass { pub fn inner (& self) -> TypeDef { TypeDef (self . row (0)) } pub fn outer (& self) -> TypeDef { TypeDef (self . row (1)) } }
    };
}

impl_102!();