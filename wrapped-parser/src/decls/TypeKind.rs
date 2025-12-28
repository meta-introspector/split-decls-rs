macro_rules! deps {
    () => {
        EnumType!();
        InterfaceType!();
        ObjectType!();
        InputObjectType!();
        UnionType!();
    };
}

macro_rules! TypeKind {
    () => {
        deps!();
        # [doc = " A kind of type; scalar, object, enum, etc."] # [derive (Debug , Clone)] pub enum TypeKind { # [doc = " A scalar type."] Scalar , # [doc = " An object type."] Object (ObjectType) , # [doc = " An interface type."] Interface (InterfaceType) , # [doc = " A union type."] Union (UnionType) , # [doc = " An enum type."] Enum (EnumType) , # [doc = " An input object type."] InputObject (InputObjectType) , }
    };
}

TypeKind!();