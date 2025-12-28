macro_rules! deps {
    () => {
        ModuleDefId!();
        Item!();
        ImportOrExternCrate!();
    };
}

macro_rules! TypesItem {
    () => {
        deps!();
        pub type TypesItem = Item < ModuleDefId , ImportOrExternCrate > ;
    };
}

TypesItem!()