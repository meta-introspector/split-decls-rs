macro_rules! deps {
    () => {
        InputObjectType!();
    };
}

macro_rules! OneofObjectType {
    () => {
        deps!();
        # [doc = " A GraphQL oneof input object."] pub trait OneofObjectType : InputObjectType { }
    };
}

OneofObjectType!()