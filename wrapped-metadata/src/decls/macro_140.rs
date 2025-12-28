macro_rules! deps {
    () => {
        TypeSpec!();
        TypeDef!();
        TypeRef!();
    };
}

macro_rules! macro_140 {
    () => {
        deps!();
        code ! { TypeDefOrRef (2) (TypeDef , 0) (TypeRef , 1) (TypeSpec , 2) }
    };
}

macro_140!();