macro_rules! deps {
    () => {
        Struct!();
    };
}

macro_rules! ContainerKind {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub enum ContainerKind { Struct , TupleStruct , UnitStruct , StructVariant , TupleVariant , UnitVariant , }
    };
}

ContainerKind!()