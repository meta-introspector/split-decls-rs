macro_rules! deps {
    () => {
        TypeRef!();
        Attribute!();
        TypeDef!();
        Constant!();
        Field!();
        InterfaceImpl!();
        ClassLayout!();
        NestedClass!();
        GenericParam!();
        TypeSpec!();
        Module!();
        MemberRef!();
        ModuleRef!();
        AssemblyRef!();
        MethodDef!();
        ImplMap!();
    };
}

macro_rules! macro_127 {
    () => {
        deps!();
        tables ! { (Attribute , 1) (ClassLayout , 16) (Constant , 0) (Field , 2) (GenericParam , 3) (ImplMap , 11) (InterfaceImpl , 4) (MemberRef , 5) (MethodDef , 6) (ModuleRef , 12) (NestedClass , 13) (MethodParam , 7) (TypeDef , 8) (TypeRef , 9) (TypeSpec , 10) (Module , 0) (AssemblyRef , 0x23) }
    };
}

macro_127!();