macro_rules! deps {
    () => {
        ContainerType!();
    };
}

macro_rules! UnionType {
    () => {
        deps!();
        # [doc = " A GraphQL interface."] pub trait UnionType : ContainerType { }
    };
}

UnionType!()