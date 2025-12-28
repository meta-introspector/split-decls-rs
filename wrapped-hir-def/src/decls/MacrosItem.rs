macro_rules! deps {
    () => {
        Item!();
        MacroId!();
        ImportOrExternCrate!();
    };
}

macro_rules! MacrosItem {
    () => {
        deps!();
        pub type MacrosItem = Item < MacroId , ImportOrExternCrate > ;
    };
}

MacrosItem!()