macro_rules! deps {
    () => {
        Static!();
        AssocItemLoc!();
    };
}

macro_rules! StaticLoc {
    () => {
        deps!();
        pub type StaticLoc = AssocItemLoc < ast :: Static > ;
    };
}

StaticLoc!();