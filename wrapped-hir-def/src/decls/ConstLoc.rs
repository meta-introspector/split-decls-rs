macro_rules! deps {
    () => {
        AssocItemLoc!();
        Const!();
    };
}

macro_rules! ConstLoc {
    () => {
        deps!();
        type ConstLoc = AssocItemLoc < ast :: Const > ;
    };
}

ConstLoc!();