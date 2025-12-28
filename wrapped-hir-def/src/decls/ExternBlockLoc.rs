macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! ExternBlockLoc {
    () => {
        deps!();
        type ExternBlockLoc = ItemLoc < ast :: ExternBlock > ;
    };
}

ExternBlockLoc!()