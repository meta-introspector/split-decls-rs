macro_rules! deps {
    () => {
        Attribute!();
        MemberRef!();
        ClassLayout!();
        ImplMap!();
        Assembly!();
        Constant!();
        Module!();
        ModuleRef!();
        MethodDef!();
        NestedClass!();
        InterfaceImpl!();
        Param!();
        TypeDef!();
        AssemblyRef!();
        TypeRef!();
        TypeSpec!();
        GenericParam!();
        Field!();
    };
}

macro_rules! Records {
    () => {
        deps!();
        # [derive (Default)] pub struct Records { pub Assembly : Vec < Assembly > , pub AssemblyRef : Vec < AssemblyRef > , pub Attribute : Vec < Attribute > , pub ClassLayout : Vec < ClassLayout > , pub Constant : Vec < Constant > , pub Field : Vec < Field > , pub GenericParam : Vec < GenericParam > , pub ImplMap : Vec < ImplMap > , pub InterfaceImpl : Vec < InterfaceImpl > , pub MemberRef : Vec < MemberRef > , pub MethodDef : Vec < MethodDef > , pub Module : Vec < Module > , pub ModuleRef : Vec < ModuleRef > , pub NestedClass : Vec < NestedClass > , pub Param : Vec < Param > , pub TypeDef : Vec < TypeDef > , pub TypeRef : Vec < TypeRef > , pub TypeSpec : Vec < TypeSpec > , }
    };
}

Records!();