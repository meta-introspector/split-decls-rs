macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! ExternCrateLoc {
    () => {
        deps!();
        type ExternCrateLoc = ItemLoc < ast :: ExternCrate > ;
    };
}

ExternCrateLoc!()