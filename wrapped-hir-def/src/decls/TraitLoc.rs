macro_rules! deps {
    () => {
        Trait!();
        ItemLoc!();
    };
}

macro_rules! TraitLoc {
    () => {
        deps!();
        pub type TraitLoc = ItemLoc < ast :: Trait > ;
    };
}

TraitLoc!()