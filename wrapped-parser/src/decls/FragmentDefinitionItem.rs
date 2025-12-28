macro_rules! deps {
    () => {
        Positioned!();
        FragmentDefinition!();
    };
}

macro_rules! FragmentDefinitionItem {
    () => {
        deps!();
        struct FragmentDefinitionItem { name : Positioned < Name > , definition : FragmentDefinition , }
    };
}

FragmentDefinitionItem!()