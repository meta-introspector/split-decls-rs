macro_rules! deps {
    () => {
        ItemLoc!();
        Union!();
    };
}

macro_rules! UnionLoc {
    () => {
        deps!();
        pub type UnionLoc = ItemLoc < ast :: Union > ;
    };
}

UnionLoc!()