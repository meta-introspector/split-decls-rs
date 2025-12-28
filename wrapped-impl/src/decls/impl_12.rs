macro_rules! deps {
    () => {
        ContainerKind!();
        Struct!();
        Variant!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl ContainerKind { fn from_struct (node : & DataStruct) -> Self { match node . fields { Fields :: Named (_) => ContainerKind :: Struct , Fields :: Unnamed (_) => ContainerKind :: TupleStruct , Fields :: Unit => ContainerKind :: UnitStruct , } } fn from_variant (node : & syn :: Variant) -> Self { match node . fields { Fields :: Named (_) => ContainerKind :: StructVariant , Fields :: Unnamed (_) => ContainerKind :: TupleVariant , Fields :: Unit => ContainerKind :: UnitVariant , } } }
    };
}

impl_12!()