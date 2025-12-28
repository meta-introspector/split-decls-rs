macro_rules! deps {
    () => {
        InputType!();
    };
}

macro_rules! InputObjectType {
    () => {
        deps!();
        # [doc = " A GraphQL input object."] pub trait InputObjectType : InputType { }
    };
}

InputObjectType!()