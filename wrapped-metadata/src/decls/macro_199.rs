macro_rules! deps {
    () => {
        NestedClass!();
        ImplMap!();
        ModuleRef!();
        Param!();
        MemberRef!();
        AssemblyRef!();
        GenericParam!();
        Attribute!();
        InterfaceImpl!();
        MethodDef!();
        ClassLayout!();
        Constant!();
        TypeDef!();
        TypeSpec!();
        Module!();
        Field!();
        TypeRef!();
    };
}

macro_rules! macro_199 {
    () => {
        deps!();
        identifiers ! { Attribute ClassLayout Constant Field GenericParam ImplMap InterfaceImpl MemberRef MethodDef ModuleRef NestedClass MethodParam TypeDef TypeRef TypeSpec Module AssemblyRef Param BlobId StringId }
    };
}

macro_199!();