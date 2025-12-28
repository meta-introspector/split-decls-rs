macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! FunctionLoc {
    () => {
        deps!();
        type FunctionLoc = AssocItemLoc < ast :: Fn > ;
    };
}

FunctionLoc!()