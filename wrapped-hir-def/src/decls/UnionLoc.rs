macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! UnionLoc {
    () => {
        deps!();
        pub type UnionLoc = ItemLoc < ast :: Union > ;
    };
}

UnionLoc!()