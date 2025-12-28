macro_rules! deps {
    () => {
        AdtId!();
    };
}

macro_rules! macro_691 {
    () => {
        deps!();
        impl_from ! (StructId , UnionId , EnumId for AdtId) ;
    };
}

macro_691!()