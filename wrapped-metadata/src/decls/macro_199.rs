macro_rules! deps {
    () => {
        GenericParam!();
        MethodDef!();
        NestedClass!();
        InterfaceImpl!();
        ImplMap!();
        TypeSpec!();
        TypeRef!();
        AssemblyRef!();
        ClassLayout!();
        Field!();
        MemberRef!();
        Module!();
        TypeDef!();
        Param!();
        ModuleRef!();
        Constant!();
        Attribute!();
    };
}

macro_rules! macro_199 {
    () => {
        deps!();
        identifiers ! { Attribute ClassLayout Constant Field GenericParam ImplMap InterfaceImpl MemberRef MethodDef ModuleRef NestedClass MethodParam TypeDef TypeRef TypeSpec Module AssemblyRef Param BlobId StringId }
    };
}

macro_199!()