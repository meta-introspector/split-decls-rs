macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! TypeDef {
    () => {
        deps!();
        pub struct TypeDef { pub Flags : TypeAttributes , pub TypeName : id :: StringId , pub TypeNamespace : id :: StringId , pub Extends : TypeDefOrRef , pub FieldList : u32 , pub MethodList : u32 , }
    };
}

TypeDef!();