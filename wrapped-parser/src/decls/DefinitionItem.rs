macro_rules! deps {
    () => {
        FragmentDefinitionItem!();
        OperationDefinitionItem!();
        Positioned!();
    };
}

macro_rules! DefinitionItem {
    () => {
        deps!();
        enum DefinitionItem { Operation (Positioned < OperationDefinitionItem >) , Fragment (Positioned < FragmentDefinitionItem >) , }
    };
}

DefinitionItem!()