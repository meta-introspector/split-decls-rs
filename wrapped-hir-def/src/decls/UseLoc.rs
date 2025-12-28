macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! UseLoc {
    () => {
        deps!();
        type UseLoc = ItemLoc < ast :: Use > ;
    };
}

UseLoc!()