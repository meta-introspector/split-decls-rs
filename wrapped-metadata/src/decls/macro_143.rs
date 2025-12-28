macro_rules! deps {
    () => {
        GenericParam!();
        InterfaceImpl!();
        Field!();
        TypeRef!();
        MemberRef!();
        TypeSpec!();
        TypeDef!();
        Param!();
        MethodDef!();
    };
}

macro_rules! macro_143 {
    () => {
        deps!();
        code ! { HasAttribute (5) (MethodDef , 0) (Field , 1) (TypeRef , 2) (TypeDef , 3) (Param , 4) (InterfaceImpl , 5) (MemberRef , 6) (TypeSpec , 13) (GenericParam , 19) }
    };
}

macro_143!()