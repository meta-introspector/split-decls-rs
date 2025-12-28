macro_rules! deps {
    () => {
        OperationDefinitionItem!();
        FragmentDefinitionItem!();
        Positioned!();
    };
}

macro_rules! DefinitionItem {
    () => {
        deps!();
        enum DefinitionItem { Operation (Positioned < OperationDefinitionItem >) , Fragment (Positioned < FragmentDefinitionItem >) , }
    };
}

DefinitionItem!();