macro_rules! deps {
    () => {
        ItemLoc!();
        ExternBlock!();
    };
}

macro_rules! ExternBlockLoc {
    () => {
        deps!();
        type ExternBlockLoc = ItemLoc < ast :: ExternBlock > ;
    };
}

ExternBlockLoc!()