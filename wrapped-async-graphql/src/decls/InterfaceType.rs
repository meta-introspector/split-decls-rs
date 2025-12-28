macro_rules! deps {
    () => {
        ContainerType!();
    };
}

macro_rules! InterfaceType {
    () => {
        deps!();
        # [doc = " A GraphQL interface."] pub trait InterfaceType : ContainerType { }
    };
}

InterfaceType!()