macro_rules! OperationType {
    () => {
        # [doc = " The type of an operation; `query`, `mutation` or `subscription`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#OperationType)."] # [derive (Debug , PartialEq , Eq , Copy , Clone , Serialize , Deserialize)] pub enum OperationType { # [doc = " A query."] Query , # [doc = " A mutation."] Mutation , # [doc = " A subscription."] Subscription , }
    };
}

OperationType!()