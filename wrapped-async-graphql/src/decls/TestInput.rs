macro_rules! deps {
    () => {
        InputObject!();
    };
}

macro_rules! TestInput {
    () => {
        deps!();
        # [derive (InputObject)] # [graphql (internal)] struct TestInput { id : i32 , name : String , }
    };
}

TestInput!();