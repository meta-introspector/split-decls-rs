macro_rules! deps {
    () => {
        Display!();
        Struct!();
        ContainerKind!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Display for ContainerKind { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (match self { ContainerKind :: Struct => "struct" , ContainerKind :: TupleStruct => "tuple struct" , ContainerKind :: UnitStruct => "unit struct" , ContainerKind :: StructVariant => "struct variant" , ContainerKind :: TupleVariant => "tuple variant" , ContainerKind :: UnitVariant => "unit variant" , }) } }
    };
}

impl_13!();