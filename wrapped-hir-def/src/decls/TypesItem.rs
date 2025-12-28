macro_rules! deps {
    () => {
        ModuleDefId!();
        ImportOrExternCrate!();
        Item!();
    };
}

macro_rules! TypesItem {
    () => {
        deps!();
        pub type TypesItem = Item < ModuleDefId , ImportOrExternCrate > ;
    };
}

TypesItem!();