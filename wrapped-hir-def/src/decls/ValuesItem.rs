macro_rules! deps {
    () => {
        ImportOrGlob!();
        ModuleDefId!();
        Item!();
    };
}

macro_rules! ValuesItem {
    () => {
        deps!();
        pub type ValuesItem = Item < ModuleDefId , ImportOrGlob > ;
    };
}

ValuesItem!();