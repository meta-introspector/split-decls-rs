macro_rules! deps {
    () => {
        Mutation!();
        Object!();
        TestInput!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        # [Object (internal)] impl Mutation { async fn test_input (& self , # [graphql (default)] input : TestInput) -> i32 { unimplemented ! () } }
    };
}

impl_165!()