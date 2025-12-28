macro_rules! deps {
    () => {
        ItemLoc!();
        ExternCrate!();
    };
}

macro_rules! ExternCrateLoc {
    () => {
        deps!();
        type ExternCrateLoc = ItemLoc < ast :: ExternCrate > ;
    };
}

ExternCrateLoc!()