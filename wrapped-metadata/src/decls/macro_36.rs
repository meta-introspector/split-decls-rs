macro_rules! deps {
    () => {
        TypeDef!();
    };
}

macro_rules! macro_36 {
    () => {
        deps!();
        code ! { TypeOrMethodDef (1) (TypeDef , 0) }
    };
}

macro_36!();