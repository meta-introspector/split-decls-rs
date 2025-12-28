macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! ImplLoc {
    () => {
        deps!();
        type ImplLoc = ItemLoc < ast :: Impl > ;
    };
}

ImplLoc!()