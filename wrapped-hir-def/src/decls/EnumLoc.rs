macro_rules! deps {
    () => {
        Enum!();
        ItemLoc!();
    };
}

macro_rules! EnumLoc {
    () => {
        deps!();
        pub type EnumLoc = ItemLoc < ast :: Enum > ;
    };
}

EnumLoc!()