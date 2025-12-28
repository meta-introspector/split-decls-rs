macro_rules! deps {
    () => {
        TypeRef!();
        TypeDef!();
    };
}

macro_rules! macro_33 {
    () => {
        deps!();
        code ! { MemberRefParent (3) (TypeDef , 0) (TypeRef , 1) }
    };
}

macro_33!()