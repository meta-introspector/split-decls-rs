macro_rules! deps {
    () => {
        ExternCrate!();
        ItemLoc!();
    };
}

macro_rules! ExternCrateLoc {
    () => {
        deps!();
        type ExternCrateLoc = ItemLoc < ast :: ExternCrate > ;
    };
}

ExternCrateLoc!();