macro_rules! deps {
    () => {
        Impl!();
        ItemLoc!();
    };
}

macro_rules! ImplLoc {
    () => {
        deps!();
        type ImplLoc = ItemLoc < ast :: Impl > ;
    };
}

ImplLoc!();