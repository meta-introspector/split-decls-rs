macro_rules! deps {
    () => {
        ItemLoc!();
        Use!();
    };
}

macro_rules! UseLoc {
    () => {
        deps!();
        type UseLoc = ItemLoc < ast :: Use > ;
    };
}

UseLoc!();