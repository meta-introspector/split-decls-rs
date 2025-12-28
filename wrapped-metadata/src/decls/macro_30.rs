macro_rules! deps {
    () => {
        GenericParam!();
        InterfaceImpl!();
        TypeRef!();
        Field!();
        MemberRef!();
        TypeSpec!();
        MethodDef!();
        TypeDef!();
    };
}

macro_rules! macro_30 {
    () => {
        deps!();
        code ! { HasAttribute (5) (MethodDef , 0) (Field , 1) (TypeRef , 2) (TypeDef , 3) (MethodParam , 4) (InterfaceImpl , 5) (MemberRef , 6) (TypeSpec , 13) (GenericParam , 19) }
    };
}

macro_30!();