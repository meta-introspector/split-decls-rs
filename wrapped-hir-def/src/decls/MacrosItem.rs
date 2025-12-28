macro_rules! deps {
    () => {
        MacroId!();
        Item!();
        ImportOrExternCrate!();
    };
}

macro_rules! MacrosItem {
    () => {
        deps!();
        pub type MacrosItem = Item < MacroId , ImportOrExternCrate > ;
    };
}

MacrosItem!();