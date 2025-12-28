macro_rules! deps {
    () => {
        Positioned!();
        OperationDefinition!();
    };
}

macro_rules! DocumentOperations {
    () => {
        deps!();
        # [doc = " The operations of a GraphQL document."] # [doc = ""] # [doc = " There is either one anonymous operation or many named operations."] # [derive (Debug , Clone , Serialize , Deserialize)] pub enum DocumentOperations { # [doc = " The document contains a single anonymous operation."] Single (Positioned < OperationDefinition >) , # [doc = " The document contains many named operations."] Multiple (HashMap < Name , Positioned < OperationDefinition > >) , }
    };
}

DocumentOperations!()