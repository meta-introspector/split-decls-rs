macro_rules! deps {
    () => {
        ItemLoc!();
        Trait!();
    };
}

macro_rules! TraitLoc {
    () => {
        deps!();
        pub type TraitLoc = ItemLoc < ast :: Trait > ;
    };
}

TraitLoc!();