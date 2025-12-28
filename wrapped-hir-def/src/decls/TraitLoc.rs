macro_rules! deps {
    () => {
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