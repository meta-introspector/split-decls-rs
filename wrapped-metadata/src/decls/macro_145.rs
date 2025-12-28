macro_rules! deps {
    () => {
        TypeRef!();
    };
}

macro_rules! macro_145 {
    () => {
        deps!();
        code ! { MemberRefParent (3) (TypeRef , 1) }
    };
}

macro_145!();