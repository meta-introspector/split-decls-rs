macro_rules! deps {
    () => {
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