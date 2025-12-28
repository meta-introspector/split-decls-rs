macro_rules! deps {
    () => {
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