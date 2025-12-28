macro_rules! deps {
    () => {
        TypeDef!();
        TypeRef!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl MemberRefParent < '_ > { pub fn namespace (& self) -> & str { match self { Self :: TypeDef (row) => row . namespace () , Self :: TypeRef (row) => row . namespace () , } } pub fn name (& self) -> & str { match self { Self :: TypeDef (row) => row . name () , Self :: TypeRef (row) => row . name () , } } }
    };
}

impl_34!();