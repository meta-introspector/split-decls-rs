macro_rules! deps {
    () => {
        ItemLoc!();
        Impl!();
    };
}

macro_rules! ImplLoc {
    () => {
        deps!();
        type ImplLoc = ItemLoc < ast :: Impl > ;
    };
}

ImplLoc!()