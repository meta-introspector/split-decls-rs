macro_rules! deps {
    () => {
        ItemLoc!();
        Enum!();
    };
}

macro_rules! EnumLoc {
    () => {
        deps!();
        pub type EnumLoc = ItemLoc < ast :: Enum > ;
    };
}

EnumLoc!();