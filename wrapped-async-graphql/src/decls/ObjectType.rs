macro_rules! deps {
    () => {
        ContainerType!();
    };
}

macro_rules! ObjectType {
    () => {
        deps!();
        # [doc = " A GraphQL object."] pub trait ObjectType : ContainerType { }
    };
}

ObjectType!();