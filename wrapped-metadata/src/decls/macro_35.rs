macro_rules! deps {
    () => {
        TypeRef!();
        TypeSpec!();
        TypeDef!();
    };
}

macro_rules! macro_35 {
    () => {
        deps!();
        code ! { TypeDefOrRef (2) (TypeDef , 0) (TypeRef , 1) (TypeSpec , 2) }
    };
}

macro_35!()