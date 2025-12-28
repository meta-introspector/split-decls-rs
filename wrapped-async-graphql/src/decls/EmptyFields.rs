macro_rules! EmptyFields {
    () => {
        # [doc = " Empty additional fields"] # [derive (SimpleObject)] # [graphql (internal , fake)] pub struct EmptyFields ;
    };
}

EmptyFields!();