macro_rules! deps {
    () => {
        Const!();
        AssocItemLoc!();
    };
}

macro_rules! ConstLoc {
    () => {
        deps!();
        type ConstLoc = AssocItemLoc < ast :: Const > ;
    };
}

ConstLoc!()