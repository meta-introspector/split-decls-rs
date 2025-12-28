macro_rules! deps {
    () => {
        Item!();
        ModuleDefId!();
        ImportOrGlob!();
    };
}

macro_rules! ValuesItem {
    () => {
        deps!();
        pub type ValuesItem = Item < ModuleDefId , ImportOrGlob > ;
    };
}

ValuesItem!()