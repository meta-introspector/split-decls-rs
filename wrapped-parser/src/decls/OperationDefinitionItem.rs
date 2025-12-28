macro_rules! deps {
    () => {
        Positioned!();
        OperationDefinition!();
    };
}

macro_rules! OperationDefinitionItem {
    () => {
        deps!();
        struct OperationDefinitionItem { name : Option < Positioned < Name > > , definition : OperationDefinition , }
    };
}

OperationDefinitionItem!()